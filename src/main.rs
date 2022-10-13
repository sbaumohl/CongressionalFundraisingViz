pub mod entities;
mod schema;

use citizensdivided::EnvConfig;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::{get, launch, routes, State};
use schema::*;

use async_graphql_rocket::GraphQLRequest;
use sea_orm::Database;

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
    request.execute(schema).await
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, World!"
}

#[get("/congress")]
fn congress(congress_no: &State<String>) -> String {
    congress_no.to_string()
}

#[launch]
async fn rocket() -> _ {
    let config = EnvConfig::new();

    let connection = Database::connect(&config.database_url)
        .await
        .expect("Error initializing DB connnection");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(connection.clone())
        .finish();

    rocket::build()
        .manage(connection)
        .manage(schema)
        .manage(config.congress)
        .mount("/", routes![index, graphql_request, congress])
}
