extern crate dotenv;
use citizensdivided::bulk_data::fec_data_handler;
use citizensdivided::entities::{prelude::*};
use dotenv::dotenv;
use sea_orm::{ActiveValue, Database, DatabaseConnection, EntityTrait};
use std::env;

mod propublica_request_handler {
    use citizensdivided::entities::members;
    use sea_orm::{ActiveModelTrait, ActiveValue};
    use serde::Deserialize;
    use serde_json::{json, Error, Value};
    use url::Url;
    const PROPUBLICA_URL: &str = "https://api.propublica.org/congress/v1/";

    #[derive(Deserialize)]
    pub struct PropublicaReturn {
        pub status: String,
        pub copyright: String,
        pub results: Vec<Value>,
    }

    async fn get(
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

    pub async fn get_candidates(
        client: &reqwest::Client,
        api_key: &str,
        chamber: &str,
        congress_no: &str,
    ) -> Vec<members::ActiveModel> {
        let returned_candidates = get(
            client,
            &format!(
                "{congress}/{chamber}/members.json?in_office=True",
                congress = congress_no,
                chamber = chamber
            ),
            api_key,
        )
        .await
        .expect("Error Requesting Senate Members");

        let members = returned_candidates.results[0]["members"]
            .as_array()
            .expect("Error Deserializing JSON!")
            .to_vec();

        let mut member_models: Vec<members::ActiveModel> = members
            .iter()
            .map(|x| {
                members::ActiveModel::from_json(json!(x))
                    .expect("Conversion to Model Object Failed")
            })
            .collect();

        for member in member_models.iter_mut() {
            (*member).chamber = ActiveValue::set(Some(chamber.to_owned()));
        }

        return member_models;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let propublica_key = env::var("PROPUBLICA_KEY").expect("PROPUBLICA_KEY must be set");
    let congress_no = env::var("CONGRESS").expect("CONGRESS must be set");

    let test_run = false;

    let connection: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Error initializing DB connnection");

    let client = reqwest::Client::new();

    // get congressional bio info for current members
    let senate_members_vec = propublica_request_handler::get_candidates(
        &client,
        &propublica_key,
        &"senate",
        &congress_no,
    )
    .await;

    let house_members_vec = propublica_request_handler::get_candidates(
        &client,
        &propublica_key,
        &"house",
        &congress_no,
    )
    .await;

    let mut congress_members_vec = [senate_members_vec, house_members_vec].concat();

    for member in congress_members_vec.iter_mut() {
        if (*member).next_election.is_not_set() {
            (*member).next_election = ActiveValue::set(None);
        }

        if (*member).geoid.is_not_set() {
            (*member).geoid = ActiveValue::Set(None);
        }

        if member.to_owned().chamber.unwrap().unwrap() == "senate" {
            (*member).district = ActiveValue::Set(None);
            (*member).at_large = ActiveValue::Set(None);
        }
        if member.fec_candidate_id.to_owned().unwrap().is_none()
            || member.fec_candidate_id.to_owned().unwrap().unwrap() == ""
        {
            (*member).fec_candidate_id = ActiveValue::Set(None);
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

    //  push to DB
    if !test_run {
        for x in congress_members_vec.iter() {
            if x.fec_candidate_id.clone().unwrap().unwrap_or_default() == "" {
                println!("{:?}", x);
            }
        }
        let res = Members::insert_many(congress_members_vec.clone())
            .exec(&connection)
            .await;
        println!("Pushed Members to DB! {:?}", res);
    }

    // scrape data from data downloads

    // this vec helps me ensure all committees comply with the foreign key constraint in the db

    let member_ids: Vec<String> = congress_members_vec
        .iter()
        .map(|c| c.fec_candidate_id.to_owned().unwrap().unwrap_or_default())
        .collect();

    // let ind_expenditures = fec_data_handler::parse_bulk_committee_to_candidate_data();
    let mut committees = fec_data_handler::parse_bulk_committees();

    committees.retain(|e| {
        e.candidate_id.to_owned().unwrap().is_none()
            || member_ids.contains(&e.candidate_id.to_owned().unwrap().unwrap())
    });

    if !test_run {
        let committees_upload_response =
            Committees::insert_many(committees[0..5000].to_vec()).exec(&connection).await;

        match committees_upload_response {
            Ok(f) => println!("Pushed Committees to DB!"),
            Err(e) => println!("Error pushing committees {}", e),
        }

        // let independent_expenditures_response =
        //     IndependentExpenditures::insert_many(ind_expenditures);
    }
}
