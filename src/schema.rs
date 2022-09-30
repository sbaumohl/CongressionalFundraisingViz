use async_graphql::{ComplexObject, Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

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

    async fn committees(&self, ctx:&Context<'_>) -> Result<Vec<committees::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Committees::find().all(db).await
    }
    
}