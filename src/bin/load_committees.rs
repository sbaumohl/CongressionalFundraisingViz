extern crate dotenv;
use citizensdivided::fec_data::{get_sorted_path_bufs, none_if_empty, page_data, new_file_reading_progress_spinner};
use citizensdivided::entities::{committees, members, prelude::*};
use citizensdivided::EnvConfig;

use sea_orm::{ActiveValue, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

/// * Reads through each file in /bin/committees line by line, parsing all committee filings, and loading them into our object models. We merge filings with the same Id, preferring the newer filing.
///
pub fn parse_bulk_committees() -> Vec<committees::ActiveModel> {
    // Data schema: https://www.fec.gov/campaign-finance-data/committee-master-file-description/

    let paths = get_sorted_path_bufs("committees/");

    let mut committees: HashMap<String, committees::ActiveModel> = HashMap::new();

    for path in paths {
        let file = File::open(&path).expect("error reading file");
        let lines = io::BufReader::new(file).lines();

        let spinner = new_file_reading_progress_spinner(path);

        for line in spinner.wrap_iter(lines) {
            if let Ok(ip) = line {
                let row = ip.split('|').collect::<Vec<&str>>();

                let id = row[0].to_string();

                let new_committee = committees::ActiveModel {
                    id: ActiveValue::Set(id.clone()),
                    name: ActiveValue::Set(row[1].to_string()),
                    designation: ActiveValue::Set(row[8].to_string()),
                    org_type: ActiveValue::Set(row[12].to_string()),
                    connected_org: ActiveValue::Set(none_if_empty(row[13])),
                    candidate_id: ActiveValue::Set(none_if_empty(row[14])),
                };

                committees.insert(id, new_committee);
            }
        }

        spinner.finish();
    }
    return committees.values().cloned().collect();
}

#[tokio::main]
async fn main() {
    let config = EnvConfig::new();

    let connection: DatabaseConnection = Database::connect(config.database_url)
        .await
        .expect("Error initializing DB connnection");

    // drop current committees
    match Committees::delete_many().exec(&connection).await {
        Ok(res) => println!(
            "Success deleting all current committees: {:?}. NOTE: THIS CASCADE DELETES AS WELL!",
            res
        ),
        Err(e) => println!("Error deleting current committees: {:?}", e),
    }

    let member_ids: Vec<String> = Members::find()
        .filter(members::Column::FecCandidateId.is_not_null())
        .all(&connection)
        .await
        .expect("Error retrieving current members in DB!")
        .iter()
        .map(|c| c.fec_candidate_id.to_owned().unwrap())
        .collect();

    let mut committees = parse_bulk_committees();

    committees.retain(|e| {
        e.candidate_id.to_owned().unwrap().is_none()
            || member_ids.contains(&e.candidate_id.to_owned().unwrap().unwrap())
    });

    for paginated_rows in page_data(committees, 10_000) {
        let committees_upload_response = Committees::insert_many(paginated_rows)
            .exec(&connection)
            .await;

        match committees_upload_response {
            Ok(f) => println!("Pushed Committees to DB! {:?}", f),
            Err(e) => {
                println!("Error pushing committees {}", e);
                break;
            }
        }
    }
}
