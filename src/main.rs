use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use citizensdivided::*;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use sea_orm::Database;

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    // load config
    let config = EnvConfig::new();

    // initialize async debugger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    
    // init db connection
    let database = Database::connect(config.database_url).await.unwrap();

    // init dataloader
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );

    // build graphql schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(database)
        .data(orm_dataloader);
    
    // set graphql max complexity and depth
    let schema = if let Some(depth) = config.depth_limit {
        schema.limit_depth(depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = config.complexity_limit {
        schema.limit_complexity(complexity)
    } else {
        schema
    };

    let schema = schema.finish();

    let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));
    
    println!("Playground: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
        .unwrap();
}
