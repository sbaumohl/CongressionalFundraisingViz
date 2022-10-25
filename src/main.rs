pub mod entities;
mod schema;

use citizensdivided::{EnvConfig, entities::prelude::Members};

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_rocket::*;
use rocket::{get, launch, routes, State};
use schema::*;

use async_graphql_rocket::GraphQLRequest;
use sea_orm::{Database, EntityTrait, QuerySelect, ColumnTrait};
use crate::entities::{prelude::*, *};

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
fn congress(congress: &State<u8>) -> String {
    congress.to_string()
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

    // https://docs.rs/sea-orm/0.9.3/sea_orm/entity/prelude/trait.ColumnTrait.html#method.is_in
    // let query = Members::find().column(members::Column::Id.is_in(vec![""])).all(&db);

    rocket::build()
        .manage(connection)
        .manage(schema)
        .manage(config.congress)
        .mount("/", routes![index, graphql_request, congress])
}
