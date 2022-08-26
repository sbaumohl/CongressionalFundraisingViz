extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub mod db_connection_handler {
    use sqlx::{postgres::PgConnection, Connection};

    pub async fn get_connection(url: &str) -> PgConnection {
        let connection = PgConnection::connect(url).await;
        match connection {
            Err(e) => panic!("Connection Error: {}", e),
            Ok(f) => return f,
        }
    }
}

mod propublica_request_handler {
    use url::Url;
    const PROPUBLICA_URL: &str = "https://api.propublica.org/congress/v1/";

    pub async fn get(client: reqwest::Client, endpoint: &str, api_key: &str) -> String {
        let url = match Url::parse(PROPUBLICA_URL) {
            Ok(url) => match url.join(endpoint) {
                Ok(url) => url,
                Err(e) => panic!("{}", e),
            },
            Err(e) => panic!("{}", e),
        };

        match client
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

    let connection = db_connection_handler::get_connection(&database_url).await;

    let client = reqwest::Client::new();

    let house_response = propublica_request_handler::get(
        client,
        &format!(
            "{congress}/{chamber}/members.json?in_office=True",
            congress = &congress_no,
            chamber = &"senate"
        ),
        &propublica_key,
    )
    .await;

    println!("{}", house_response);
}
