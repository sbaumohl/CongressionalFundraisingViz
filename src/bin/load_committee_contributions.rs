use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use citizensdivided::entities::{prelude::*, *};
use citizensdivided::{fec_data, EnvConfig};
use sea_orm::{
    ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};

/// * Reads through each file in /bin/contributions_from_committee_ind_expenditures line by line, parsing all items marked as committee disbursements to other committees, and loading them into our object model. We merge lines that reference the same candidates and committees within the same election cycle.
/// * Also note that each filing has a date it was filed, and the election its marked for. We treat its filing date as its "election cycle".
///
pub fn parse_bulk_committee_disbursements() -> Vec<committee_contributions::ActiveModel> {
    let paths = fec_data::get_sorted_path_bufs("contributions_from_committee_ind_expenditures/");

    let mut committee_contributions: Vec<committee_contributions::ActiveModel> = Vec::new();

    for path in paths {
        let file = File::open(&path).expect("error reading file");
        let lines = io::BufReader::new(file).lines();

        let mut mapped_rows: HashMap<String, committee_contributions::ActiveModel> = HashMap::new();

        // progress bar
        let bar = fec_data::new_file_reading_progress_spinner(path);

        for line in bar.wrap_iter(lines) {
            if let Ok(ip) = line {
                let row = ip.split('|').collect::<Vec<&str>>();

                if !row[5].eq("24K") {
                    continue;
                }

                let election_cycle = fec_data::parse_filing_date(row[4]);
                let sender_committee = row[0].to_string();
                let recipient_committee = row[15].to_string();
                let recipient_candidate = row[16].to_string();
                let amount = row[14].parse::<i32>().unwrap();

                // Use Hashmap (with the key being a composite of properties we want to merge) to turn O(n) lookup to O(k)
                let key = format!(
                    "{} to {}-{}",
                    sender_committee, recipient_committee, recipient_candidate
                );

                let committee_disbursement = committee_contributions::ActiveModel {
                    id: ActiveValue::NotSet,
                    spender_committee: ActiveValue::Set(sender_committee),
                    election_cycle: ActiveValue::Set(election_cycle),
                    recipient_committee: ActiveValue::Set(recipient_committee),
                    recipient_candidate: ActiveValue::Set(recipient_candidate),
                    amount: ActiveValue::Set(0),
                };

                // get the entry at key, if it doesn't exist, add a new entry with amount 0.
                // then, using that reference, increment the amount value. This ensures all unadded entries are added w/o duplicates.
                let disbursement = mapped_rows.entry(key).or_insert(committee_disbursement);
                let new_amt = disbursement.amount.as_ref() + amount;
                (*disbursement).amount = ActiveValue::Set(new_amt);
            }
        }
        committee_contributions.append(&mut Vec::from_iter(mapped_rows.values().cloned()));
        bar.finish();
    }
    return committee_contributions;
}

#[tokio::main]
async fn main() {
    let config = EnvConfig::new();

    let connection: DatabaseConnection = Database::connect(&config.database_url)
        .await
        .expect("Error initializing DB connnection");

    match CommitteeContributions::delete_many()
        .exec(&connection)
        .await
    {
        Ok(res) => println!(
            "Success deleting all committee contributions: {:?}. NOTE: THIS CASCADE DELETES AS WELL!",
            res
        ),
        Err(e) => println!("Error deleting current committees: {:?}", e),
    }

    let mut committee_contributions = parse_bulk_committee_disbursements();

    println!("Removing all entries that reference non-existent committees or candidates... This may take a while...");

    let member_ids: Vec<String> = Members::find()
        .filter(members::Column::FecCandidateId.is_not_null())
        .all(&connection)
        .await
        .expect("Error retrieving current members in DB!")
        .iter()
        .map(|c| c.fec_candidate_id.to_owned().unwrap())
        .collect();

    let committee_ids: Vec<String> = Committees::find()
        .column(committees::Column::Id)
        .all(&connection)
        .await
        .expect("Error retrieving current committees in DB!")
        .iter()
        .map(|c| c.id.to_owned())
        .collect();

    // remove rows that reference non-existent candidates or committees
    // also remove rows with 0 amounts, which happens when a amendment report cancels out a reported disbursement
    committee_contributions.retain(|e| {
        e.amount.clone().unwrap() != 0
            && committee_ids.contains(&e.spender_committee.to_owned().unwrap())
            && committee_ids.contains(&e.recipient_committee.to_owned().unwrap())
            && member_ids.contains(&e.recipient_candidate.to_owned().unwrap())
    });

    for part in fec_data::page_data(committee_contributions, 10_000) {
        let response = CommitteeContributions::insert_many(part)
            .exec(&connection)
            .await;

        match response {
            Ok(f) => println!("Pushed Committee Contributions to DB! {:?}", f),
            Err(e) => {
                println!("Error pushing Committee Contributions {}", e);
                break;
            }
        }
    }
}
