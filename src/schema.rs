use async_graphql::{Context, Object, InputType, InputObject, dataloader::{DataLoader, Loader}, async_trait, futures_util::TryFutureExt};
use sea_orm::*;

use crate::entities::{prelude::*, *};


pub(crate) struct QueryRoot;

#[derive(InputObject)]
struct SelectMember {
    fec_candidate_id: Option<String>,
    chamber: Option<String>,
    gender: Option<String>,
    party: Option<String>,
    state: Option<String>,
    district: Option<String>,
}

#[Object]
impl QueryRoot {
    async fn members(
        &self,
        ctx: &Context<'_>,
        select: Option<SelectMember>,
    ) -> async_graphql::Result<Vec<members::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();

        match select {
            None => Members::find(),
            Some(select) => {
                let mut f = Members::find();
                if let Some(fec_candidate_id) = select.fec_candidate_id {
                    f = f.filter(members::Column::FecCandidateId.eq(fec_candidate_id));
                }

                if let Some(chamber) = select.chamber {
                    f = f.filter(members::Column::Chamber.eq(chamber));
                }

                if let Some(gender) = select.gender {
                    f = f.filter(members::Column::Gender.eq(gender));
                }

                if let Some(party) = select.party {
                    f = f.filter(members::Column::Party.eq(party));
                }

                if let Some(state) = select.state {
                    f = f.filter(members::Column::State.eq(state));
                }

                if let Some(district) = select.district {
                    f = f.filter(members::Column::District.eq(district))
                }

                f
            },
        }
        .all(db)
        .await
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

    async fn committee_contributions(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<committee_contributions::Model>, DbErr> {
        let db = ctx.data::<DatabaseConnection>().unwrap();
        CommitteeContributions::find().all(db).await
    }
}
