use async_graphql::{Context, Object};
use sea_orm::*;

use crate::entities::{prelude::*, *};

pub(crate) struct QueryRoot;

#[Object]
impl QueryRoot {

    async fn members(
        &self,
        ctx: &Context<'_>,
        fec_candidate_id: Option<String>,
    ) -> async_graphql::Result<Vec<members::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        match fec_candidate_id {
            Some(id) => {
                Members::find()
                    .filter(members::Column::FecCandidateId.eq(id))
                    .all(db)
                    .await
            }
            None => Members::find().all(db).await,
        }
    }

    async fn committees<'a>(&self, ctx: &Context<'a>) -> Result<Vec<committees::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        Committees::find().all(db).await
    }

    async fn independent_expenditures(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<independent_expenditures::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        IndependentExpenditures::find().all(db).await
    }
}
