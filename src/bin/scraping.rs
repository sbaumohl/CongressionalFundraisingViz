extern crate dotenv;
use citizensdivided::entities::{members, prelude::Members};
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::env;

mod fec_data_handler {
    use std::{
        fs::{self, File},
        io::{self, BufRead},
        path::Path,
    };

    const DATA_PATH: &str = "./src/bin/";

    pub struct CommitteeToCandidateData {
        committee: String,
        expenditure_amount: i32,
        support_or_oppose: OpposeSupportIndicator,
        election_cycle: String,
        candidate_fec_id: String,
    }

    pub enum OpposeSupportIndicator {
        Support,
        Oppose,
    }

    pub fn parse_bulk_committee_to_candidate_data() -> Vec<CommitteeToCandidateData> {
        /*
        * Data from: https://www.fec.gov/data/browse-data/?tab=bulk-data
        * Data schema: https://www.fec.gov/campaign-finance-data/contributions-committees-candidates-file-description/
        */
        let root_path = Path::new(DATA_PATH).join("contributions_from_committee_ind_expenditures/");
        let paths = fs::read_dir(root_path).unwrap();

        for path in paths {
            let file = File::open(path.expect("").path()).expect("error reading file");
            if let lines = io::BufReader::new(file).lines() {
                
                for line in lines {
                    if let Ok(ip) = line {
                        let row = ip.split('|').collect::<Vec<&str>>();

                    }
                }
            }
        }

        return vec![];
    }
}

mod open_fec_handler {
    use serde::Deserialize;
    use serde_json::{Error, Value};
    use url::Url;

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    const OPEN_FEC_URL: &str = "https://api.open.fec.gov/v1/";

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

    async fn get(
        client: &reqwest::Client,
        endpoint: &str,
        api_key: &str,
    ) -> Result<OpenFecReturn, Error> {
        let base_url = Url::parse(OPEN_FEC_URL).expect("Open FEC base url broken");

        let url = base_url
            .join(endpoint)
            .expect("Error Joining FEC URL with given Endpoint");

        let json = client
            .get(url.as_str())
            .header("x-api-key", api_key)
            .send()
            .await
            .expect("Error when requesting FEC data")
            .text()
            .await
            .expect("Error Decoding JSON to text");

        let decoded_json =
            serde_json::from_str(&json).expect("Error Decoding JSON to serde_json object");

        Ok(decoded_json)
    }

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
        let base_url = Url::parse(PROPUBLICA_URL).expect("Propublica base url broken");

        let url = base_url
            .join(endpoint)
            .expect("Error Joining Propublica URL with given Endpoint");

        let json = client
            .get(url.as_str())
            .header("x-api-key", api_key)
            .send()
            .await
            .expect("Error when requesting Propublica data")
            .text()
            .await
            .expect("Error Decoding JSON to text");

        let decoded_json =
            serde_json::from_str(&json).expect("Error Decoding JSON to serde_json object");

        Ok(decoded_json)
    }
}

#[tokio::main]
async fn main() {
    // open_fec_handler::read_table();
    fec_data_handler::parse_bulk_committee_to_candidate_data();
    return;
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
