use sea_orm::entity::prelude::*;

/// Independent Expenditures or Schedule E filings are advertising that political action committees pay for to advocate for certain issues or candidates. 
/// They have to disclose which candidate they are advertising for, 
///  whether it is a statement of support or opposition and how much is being spent.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "independent_expenditures")]
#[graphql(complex)]
#[graphql(name = "IndependentExpenditures")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub spender_committee: String,
    pub amount: i32,
    pub support_oppose: String,
    pub election_cycle: String,
    pub recipient_candidate: String,
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
