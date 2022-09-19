//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use async_graphql::{SimpleObject};
use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, SimpleObject)]
#[sea_orm(table_name = "members")]
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
    pub fec_candidate_id: String,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::committees::Entity")]
    Committee,
}

impl Related<super::committees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Committee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
