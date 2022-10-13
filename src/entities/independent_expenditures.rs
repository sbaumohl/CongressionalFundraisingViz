//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use async_graphql::{SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "IndependentExpenditures")]
#[sea_orm(table_name = "independent_expenditures")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub spender_committee: String,
    pub amount: i32,
    pub support_oppose: String,
    pub election_cycle: String,
    pub recipient_candidate: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::members::Entity",
        from = "Column::RecipientCandidate",
        to = "super::members::Column::FecCandidateId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Members,
    #[sea_orm(
        belongs_to = "super::committees::Entity",
        from = "Column::SpenderCommittee",
        to = "super::committees::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Committees,
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl Related<super::committees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Committees.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
