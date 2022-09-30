extern crate dotenv;
use citizensdivided::entities::prelude::*;
use citizensdivided::{bulk_data::fec_data_handler, entities::members};
use dotenv::dotenv;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};
use std::{env, cmp};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection: DatabaseConnection = Database::connect(&database_url)
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

    let members_vec = Members::find()
        .filter(members::Column::FecCandidateId.is_not_null())
        .all(&connection)
        .await
        .expect("Error retrieving current members in DB!");

    let member_ids: Vec<String> = members_vec
        .iter()
        .map(|c| c.fec_candidate_id.to_owned().unwrap())
        .collect();

    let mut committees = fec_data_handler::parse_bulk_committees();

    committees.retain(|e| {
        e.candidate_id.to_owned().unwrap().is_none()
            || member_ids.contains(&e.candidate_id.to_owned().unwrap().unwrap())
    });

    // Postgres gets mad if I push more than ~10,000 rows at once
    // this splits the inserts into 10,000 row slices
    let mut left = 0;
    let mut right = 0;
    loop {
        left = right;
        right = cmp::min(right + 10_000, committees.len());

        let committees_upload_response = Committees::insert_many(committees[left..right].to_vec())
            .exec(&connection)
            .await;

        match committees_upload_response {
            Ok(f) => println!("Pushed Committees (Index {} to {}) to DB! {:?}", left, right - 1, f),
            Err(e) => println!("Error pushing committees {}", e),
        }

        if right >= committees.len() - 1 {
            println!("Pushed all Committees! (size {})", committees.len());
            break;
        }
    }
}
