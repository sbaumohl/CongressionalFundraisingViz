use sea_orm::entity::prelude::*;

/// Committee Contribution represents the transfers from non-affiliated Political Action Committees to Candidate Committees.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "committee_contributions")]
#[graphql(complex)]
#[graphql(name = "CommitteeContributions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub spender_committee: String,
    pub election_cycle: String,
    pub recipient_committee: String,
    pub recipient_candidate: String,
    pub amount: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
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
        from = "Column::RecipientCommittee",
        to = "super::committees::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    RecipientCommitteeDetails,
    #[sea_orm(
        belongs_to = "super::committees::Entity",
        from = "Column::SpenderCommittee",
        to = "super::committees::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SpenderCommitteeDetails,
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
