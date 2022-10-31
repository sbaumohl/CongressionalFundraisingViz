use sea_orm::entity::prelude::*;
use serde::Deserialize;

/// Every Member is a currently sitting senator or representative in the US Congress, each Member Object holds important biographical data about each lawmaker.
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    Deserialize,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "members")]
#[graphql(complex)]
#[graphql(name = "Members")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub chamber: Option<String>,
    pub title: Option<String>,
    pub short_title: Option<String>,
    pub api_uri: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub date_of_birth: Date,
    pub gender: String,
    pub party: String,
    pub leadership_role: Option<String>,
    pub twitter_account: Option<String>,
    pub facebook_account: Option<String>,
    pub youtube_account: Option<String>,
    pub govtrack_id: Option<String>,
    pub cspan_id: Option<String>,
    pub votesmart_id: Option<String>,
    pub icpsr_id: Option<String>,
    pub crp_id: Option<String>,
    pub google_entity_id: Option<String>,
    #[sea_orm(unique)]
    pub fec_candidate_id: Option<String>,
    pub url: String,
    pub rss_url: Option<String>,
    pub contact_form: Option<String>,
    pub in_office: bool,
    pub cook_pvi: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((4, 3)))", nullable)]
    pub dw_nominate: Option<Decimal>,
    pub ideal_point: Option<String>,
    pub seniority: Option<String>,
    pub next_election: Option<String>,
    pub total_votes: Option<i32>,
    pub missed_votes: Option<i32>,
    pub total_present: Option<i32>,
    pub last_updated: String,
    pub ocd_id: Option<String>,
    pub office: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub state: String,
    pub district: Option<String>,
    pub at_large: Option<bool>,
    pub geoid: Option<String>,
    pub missed_votes_pct: Option<Decimal>,
    pub votes_with_party_pct: Option<Decimal>,
    pub votes_against_party_pct: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation,seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::committee_contributions::Entity")]
    CommitteeContributions,
    #[sea_orm(has_many = "super::committees::Entity")]
    Committees,
    #[sea_orm(has_many = "super::independent_expenditures::Entity")]
    IndependentExpenditures,
}

impl Related<super::committee_contributions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CommitteeContributions.def()
    }
}

impl Related<super::committees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Committees.def()
    }
}

impl Related<super::independent_expenditures::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::IndependentExpenditures.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
