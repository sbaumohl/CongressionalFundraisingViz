extern crate dotenv;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use citizensdivided::entities::{committees, independent_expenditures, prelude::*};
use citizensdivided::fec_data::{get_sorted_path_bufs, parse_filing_date};
use citizensdivided::EnvConfig;
use citizensdivided::{entities::members, fec_data};
use sea_orm::{
    ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};

pub fn parse_bulk_committee_to_candidate_data() -> Vec<independent_expenditures::ActiveModel> {
    // Data schema: https://www.fec.gov/campaign-finance-data/contributions-committees-candidates-file-description/

    let paths = get_sorted_path_bufs("contributions_from_committee_ind_expenditures/");

    let mut merged_data: Vec<independent_expenditures::ActiveModel> = Vec::new();

    for path in paths {
        let file = File::open(&path).expect("error reading file");
        let lines = io::BufReader::new(file).lines();

        // Use Hashmap (with the key being a composite of properties we want to merge) to turn O(n) lookup to O(k)
        let mut aggregated_rows: HashMap<String, independent_expenditures::ActiveModel> =
            HashMap::new();

        // progress bar
        let bar = fec_data::new_file_reading_progress_spinner(path);

        for line in bar.wrap_iter(lines) {
            if let Ok(ip) = line {
                let row = ip.split('|').collect::<Vec<&str>>();

                let support_or_oppose = match row[5] {
                    "24A" => "O",
                    "24E" => "S",
                    _ => continue,
                }
                .to_owned();
                let spender_committee = row[0].to_owned();
                let candidate_fec_id = row[16].to_owned();
                let election_cycle = parse_filing_date(row[4]);
                let expenditure_amt = row[14].parse::<i32>().unwrap();

                let key = format!(
                    "{} -> {} : {}",
                    spender_committee, candidate_fec_id, support_or_oppose
                );

                let item = independent_expenditures::ActiveModel {
                    id: ActiveValue::NotSet,
                    spender_committee: ActiveValue::Set(spender_committee),
                    recipient_candidate: ActiveValue::Set(candidate_fec_id),
                    support_oppose: ActiveValue::Set(support_or_oppose),
                    election_cycle: ActiveValue::Set(election_cycle),
                    amount: ActiveValue::Set(0),
                };

                // get the entry at key, if it doesn't exist, add a new entry with amount 0.
                // then, using that reference, increment the amount value. This adds all unadded entries w/o duplicates.
                let row_ref = aggregated_rows.entry(key).or_insert(item);
                let new_amt = row_ref.amount.as_ref() + expenditure_amt;
                (*row_ref).amount = ActiveValue::Set(new_amt);
            }
        }
        bar.finish();
        merged_data.append(&mut Vec::from_iter(aggregated_rows.values().cloned()));
    }

    return merged_data;
}

#[tokio::main]
async fn main() {
    let config = EnvConfig::new();

    let connection: DatabaseConnection = Database::connect(&config.database_url)
        .await
        .expect("Error initializing DB connnection");

    // drop current committees
    match IndependentExpenditures::delete_many().exec(&connection).await {
        Ok(res) => println!(
            "Success deleting all Independent Expenditures: {:?}. NOTE: THIS CASCADE DELETES AS WELL!",
            res
        ),
        Err(e) => println!("Error deleting Independent Expenditures: {:?}", e),
    }

    let member_ids: Vec<String> = Members::find()
        .filter(members::Column::FecCandidateId.is_not_null())
        .column(members::Column::FecCandidateId)
        .all(&connection)
        .await
        .expect("Error retrieving current members in DB!")
        .iter()
        .map(|c| c.fec_candidate_id.to_owned().unwrap())
        .collect();

    let committees_ids: Vec<String> = committees::Entity::find()
        .column(committees::Column::Id)
        .all(&connection)
        .await
        .expect("Error retrieving current committees in DB!")
        .iter()
        .map(|c| c.id.to_owned())
        .collect();

    let mut ind_expenditures = parse_bulk_committee_to_candidate_data();

    ind_expenditures.retain(|e| {
        member_ids.contains(&e.recipient_candidate.to_owned().unwrap())
            && committees_ids.contains(&e.spender_committee.to_owned().unwrap())
    });

    for slice in fec_data::page_data(ind_expenditures, 10_000) {
        let committees_upload_response = IndependentExpenditures::insert_many(slice)
            .exec(&connection)
            .await;

        match committees_upload_response {
            Ok(f) => println!("Pushed Independent Expenditures to DB! {:?}", f),
            Err(e) => {
                println!("Error pushing Independent Expenditures {}", e);
                break;
            }
        }
    }
}
