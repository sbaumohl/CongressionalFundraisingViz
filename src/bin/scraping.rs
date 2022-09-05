extern crate dotenv;
use dotenv::dotenv;
use sea_orm::{DatabaseConnection, DbErr, Database};
use std::env;

mod propublica_request_handler {
    use citizensdivided::entities::{members, prelude::Members};
    use sea_orm::{ActiveModelTrait, EntityTrait, DatabaseConnection, DbErr};
    use serde::Deserialize;
    use serde_json::{Value, Error, json};
    use url::Url;
    const PROPUBLICA_URL: &str = "https://api.propublica.org/congress/v1/";

    #[derive(Deserialize)]
    pub struct PropublicaReturn {
        pub status: String,
        pub copyright: String,
        pub results: Vec<Value>
    }

    pub async fn get(client: reqwest::Client, endpoint: &str, api_key: &str) -> Result<PropublicaReturn, Error> {
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

    pub async fn push_members_to_db(server_response: PropublicaReturn, db: &DatabaseConnection) -> Result<(), DbErr> {
        let senate_members = server_response.results[0]["members"].as_array().expect("Error Deserializing JSON!").to_vec();

        let members: Vec<members::ActiveModel> = senate_members.iter().map(|x| 
            members::ActiveModel::from_json(json!(x)).expect("Conversion to Model Object Failed")
        ).collect();
    
        match Members::insert_many(members).exec(db).await {
            Err(e) => Err(e),
            Ok(_) => Ok(())

        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let propublica_key = env::var("PROPUBLICA_KEY").expect("PROPUBLICA_KEY must be set");
    let fec_key = env::var("FEC_KEY").expect("PROPUBLICA_KEY must be set");
    let congress_no = env::var("CONGRESS").expect("CONGRESS must be set");

    let connection: DatabaseConnection = Database::connect(&database_url).await.expect("Error initializing DB connnection");

    let client = reqwest::Client::new();

    let senate_response = propublica_request_handler::get(
        client,
        &format!(
            "{congress}/{chamber}/members.json?in_office=True",
            congress = &congress_no,
            chamber = &"senate"
        ),
        &propublica_key,
    )
    .await.unwrap();

    match propublica_request_handler::push_members_to_db(senate_response, &connection).await {
        Err(e) => {
            println!("Error Pushing Senate Members to DB: {}", e)
        },
        Ok(_) => {
            println!("Succesfully Pushed Senate Members to DB")
        }
    };


}
