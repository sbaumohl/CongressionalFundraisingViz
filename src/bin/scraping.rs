extern crate dotenv;
use citizensdivided::entities::{members, prelude::Members};
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::env;

mod open_fec_handler {
    use serde::Deserialize;
    use serde_json::{Error, Value};
    use url::Url;

    #[derive(Deserialize)]
    struct OpenFecPagination {
        per_page: i32,
        pages: i32,
        page: i32,
        count: i32,
    }

    #[derive(Deserialize)]
    struct OpenFecReturn {
        pub api_version: String,
        pub pagination: OpenFecPagination,
        pub results: Vec<Value>,
    }

    impl OpenFecPagination {
        pub fn next_page() {
            todo!()
        }
    }

    async fn get(client: &reqwest::Client, endpoint: &str, api_key: &str) -> Result<OpenFecReturn, Error>{
        todo!();
    }

    pub async fn get_schedule_e_filings() {}

    pub async fn get_schedule_e_totals_by_candidate() {}

    pub async fn get_candidate_campaign_committees() {}
}

mod propublica_request_handler {
    use serde::Deserialize;
    use serde_json::{Error, Value};
    use url::Url;
    const PROPUBLICA_URL: &str = "https://api.propublica.org/congress/v1/";

    #[derive(Deserialize)]
    pub struct PropublicaReturn {
        pub status: String,
        pub copyright: String,
        pub results: Vec<Value>,
    }

    // TODO rewrite this!
    pub async fn get(
        client: &reqwest::Client,
        endpoint: &str,
        api_key: &str,
    ) -> Result<PropublicaReturn, Error> {
        let url = match Url::parse(PROPUBLICA_URL) {
            Ok(url) => match url.join(endpoint) {
                Ok(url) => url,
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        };

        let json = match client
            .get(url.as_str())
            .header("x-api-key", api_key)
            .send()
            .await
        {
            Err(e) => panic!("{}", e),
            Ok(res) => match res.text().await {
                Err(e) => panic!("{}", e),
                Ok(text) => text,
            },
        };

        Ok(serde_json::from_str(&json).unwrap())
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let propublica_key = env::var("PROPUBLICA_KEY").expect("PROPUBLICA_KEY must be set");
    let fec_key = env::var("FEC_KEY").expect("PROPUBLICA_KEY must be set");
    let congress_no = env::var("CONGRESS").expect("CONGRESS must be set");

    let test_run = true;

    let connection: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Error initializing DB connnection");

    let client = reqwest::Client::new();

    // get congressional bio info for current members
    let senate_members_vec = {
        let senate_response = propublica_request_handler::get(
            &client,
            &format!(
                "{congress}/{chamber}/members.json?in_office=True",
                congress = &congress_no,
                chamber = &"senate"
            ),
            &propublica_key,
        )
        .await
        .unwrap();

        let senate_members = senate_response.results[0]["members"]
            .as_array()
            .expect("Error Deserializing JSON!")
            .to_vec();

        let mut members: Vec<members::ActiveModel> = senate_members
            .iter()
            .map(|x| {
                members::ActiveModel::from_json(json!(x))
                    .expect("Conversion to Model Object Failed")
            })
            .collect();

        for member in members.iter_mut() {
            (*member).chamber = ActiveValue::set(Some("senate".to_owned()));
        }

        members
    };

    let house_rep_members_vec = {
        let house_response = propublica_request_handler::get(
            &client,
            &format!(
                "{congress}/{chamber}/members.json?in_office=True",
                congress = &congress_no,
                chamber = &"house"
            ),
            &propublica_key,
        )
        .await
        .unwrap();

        let house_members = house_response.results[0]["members"]
            .as_array()
            .expect("Error Deserializing JSON!")
            .to_vec();

        let mut members: Vec<members::ActiveModel> = house_members
            .iter()
            .map(|x| {
                members::ActiveModel::from_json(json!(x))
                    .expect("Conversion to Model Object Failed")
            })
            .collect();

        for member in members.iter_mut() {
            (*member).chamber = ActiveValue::set(Some("house".to_owned()));

            if (*member).next_election.is_not_set() {
                (*member).next_election = ActiveValue::set(None);
            }

            // The American Samoa (AS) send a Rep, but they don't vote
            // propublica leaves these values "unset" for the AS rep
            // and sea-orm breaks when some objects have values set and others dont
            if (*member).state.eq(&ActiveValue::set("AS".to_owned())) {
                (*member).missed_votes_pct = ActiveValue::set(None);
                (*member).votes_with_party_pct = ActiveValue::set(None);
                (*member).votes_against_party_pct = ActiveValue::set(None);
            }
        }

        members
    };

    let congress_members_vec = [senate_members_vec, house_rep_members_vec].concat();

    // Take JSON, and parse it into member objects, assign chamber value, push to DB
    if !test_run {
        match {
            Members::insert_many(congress_members_vec.to_vec())
                .exec(&connection)
                .await
        } {
            Err(e) => {
                println!("Error Pushing members of congress to DB: {}", e)
            }
            Ok(_) => {
                println!("Succesfully Pushed members of congress to DB")
            }
        }
    }

    for member in congress_members_vec {
        println!("{:?}", member.fec_candidate_id);
    }
}
