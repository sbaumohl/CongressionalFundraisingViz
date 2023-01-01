use async_graphql::{Context, Object};
use sea_orm::{
    sea_query::Expr, sea_query::Func, ColumnTrait, Condition, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QuerySelect,
};
use seaography::EntityFilter;

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
    async fn committees(
        &self,
        ctx: &Context<'_>,
        search: Option<String>,
        limit: Option<u64>,
        filters: Option<committees::Filter>,
    ) -> Result<Vec<committees::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        // conditional filter (search)
        let mut cond_filters = Condition::all();

        if let Some(query) = search {
            let like = format!("%{}%", query.to_lowercase());
            cond_filters = cond_filters
                .add(Expr::expr(Func::lower(Expr::col(committees::Column::Name))).like(like));
        }

        // apply filters and search
        let mut results = Committees::find().filter(cond_filters);

        // Filter Object; apply filters and search only if filters were given
        if let Some(filters) = filters {
            results = results.filter(filters.filter_condition());
        }

        // limit returned no.
        if let Some(lim) = limit {
            results = results.limit(lim);
        }

        // return results
        results.all(db).await
    }

    async fn organizations(
        &self,
        ctx: &Context<'_>,
        search: Option<String>,
        limit: Option<u64>,
        filters: Option<committees::Filter>,
    ) -> Result<Vec<String>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        // conditional filter (search)
        let mut cond_filters = Condition::all();

        // we want to exclude "NONE" or "null" connectedOrg results
        cond_filters = cond_filters
            .add(committees::Column::ConnectedOrg.is_not_null())
            .add(committees::Column::ConnectedOrg.ne("NONE"));

        if let Some(query) = search {
            println!("{:?}", query);
            let like = format!("%{}%", query.to_lowercase());
            cond_filters = cond_filters.add(
                Expr::expr(Func::lower(Expr::col(committees::Column::ConnectedOrg))).like(like),
            );
        }

        let mut results = Committees::find().filter(cond_filters);

        // Filter object; check if there are filters and apply (TODO maybe remove?)
        results = if let Some(f) = filters {
            results.filter(f.filter_condition())
        } else {
            results
        };

        // limit returned no.
        if let Some(lim) = limit {
            results = results.limit(lim);
        }

        return results.all(db).await.and_then(|f| {
            Ok(f.iter()
                .map(|f: &committees::Model| f.connected_org.clone().unwrap())
                .collect())
        });
    }

    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<members::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        Members::find().all(db).await
    }
}
