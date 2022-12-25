use async_graphql::{Context, Object};
use sea_orm::{
    sea_query::Expr, sea_query::Func, Condition, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QuerySelect,
};

use crate::entities::{prelude::*, *};

// #[derive(Debug, seaography::macros::QueryRoot)]
#[derive(Debug)]
// #[seaography(entity = "crate::entities::committee_contributions")]
// #[seaography(entity = "crate::entities::committees")]
// #[seaography(entity = "crate::entities::independent_expenditures")]
// #[seaography(entity = "crate::entities::members")]
// #[seaography(entity = "crate::entities::seaql_migrations")]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn search(&self) -> Result<String, DbErr> {
        return Ok("Hello, There!".to_string());
    }

    async fn committees(
        &self,
        ctx: &Context<'_>,
        q: Option<String>,
        limit: Option<u64>,
    ) -> Result<Vec<committees::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        // conditional filter (search)
        let mut filters = Condition::all();

        if let Some(query) = q {
            let like = format!("%{}%", query.to_lowercase());
            filters = filters
                .add(Expr::expr(Func::lower(Expr::col(committees::Column::Name))).like(like));
        }

        let mut results = Committees::find().filter(filters);

        // limit returned no.
        if let Some(lim) = limit {
            results = results.limit(lim);
        }

        // return results
        results.all(db).await
    }

    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<members::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Members::find().all(db).await
    }
}
