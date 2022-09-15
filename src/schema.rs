use async_graphql::{Object, Context};
use sea_orm::{DbErr, DatabaseConnection, EntityTrait};

use crate::entities::{members, prelude::*};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "Hello GraphQL".to_owned()
    }

    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<members::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Members::find().all(db).await
    }
}
