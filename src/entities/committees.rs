use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "committees")]
#[graphql(complex)]
#[graphql(name = "Committees")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub designation: String,
    pub org_type: String,
    pub connected_org: Option<String>,
    pub candidate_id: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::members::Entity",
        from = "Column::CandidateId",
        to = "super::members::Column::FecCandidateId",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Members,
    #[sea_orm(has_many = "super::independent_expenditures::Entity")]
    IndependentExpenditures,
}

impl Related<super::members::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl Related<super::independent_expenditures::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IndependentExpenditures.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
