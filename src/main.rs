pub mod entities;
mod schema;

extern crate dotenv;
use std::env;
use dotenv::dotenv;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::*;
use schema::*;

use async_graphql_rocket::GraphQLRequest;
use sea_orm::{Database};

type SchemaType = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(schema: &State<SchemaType>, request: GraphQLRequest) -> GraphQLResponse {
   request.execute(schema).await
}

#[get("/")]
async fn index() -> &'static str {
    "Hello, bakeries!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = Database::connect(&database_url).await.expect("Error initializing DB connnection");

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).data(connection.clone()).finish();

    rocket::build().manage(connection)
    .manage(schema)

    .mount("/", routes![index, graphql_request])

}
