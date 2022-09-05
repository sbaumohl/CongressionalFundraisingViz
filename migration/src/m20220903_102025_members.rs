use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

extern crate dotenv;
use dotenv::dotenv;
use std::env;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_members_table_sql = "CREATE TABLE IF NOT EXISTS members (
            id VARCHAR(255) NOT NULL PRIMARY KEY,
            chamber VARCHAR(255),
            title VARCHAR(255) NOT NULL,
            short_title VARCHAR(255) NOT NULL,
            api_uri VARCHAR(255) NOT NULL,
            first_name VARCHAR(255) NOT NULL,
            middle_name VARCHAR(255),
            last_name VARCHAR(255) NOT NULL,
            suffix VARCHAR(255),
            date_of_birth DATE NOT NULL,
            gender VARCHAR(255) NOT NULL,
            party VARCHAR(255) NOT NULL,
            leadership_role VARCHAR(255),
            twitter_account VARCHAR(255),
            facebook_account VARCHAR(255),
            youtube_account VARCHAR(255),
            govtrack_id VARCHAR(255),
            cspan_id VARCHAR(255),
            votesmart_id VARCHAR(255),
            icpsr_id VARCHAR(255),
            crp_id VARCHAR(255) NOT NULL,
            google_entity_id VARCHAR(255) NOT NULL,
            fec_candidate_id VARCHAR(255) NOT NULL,
            url VARCHAR(255) NOT NULL,
            rss_url VARCHAR(255),
            contact_form VARCHAR(255),
            in_office BOOLEAN NOT NULL,
            cook_pvi VARCHAR(255),
            dw_nominate NUMERIC(4, 3),
            ideal_point VARCHAR(255),
            seniority VARCHAR(255) NOT NULL,
            next_election VARCHAR(255) NOT NULL,
            total_votes INTEGER NOT NULL,
            missed_votes INTEGER NOT NULL,
            total_present INTEGER NOT NULL,
            last_updated VARCHAR(255) NOT NULL,
            ocd_id VARCHAR(255) NOT NULL,
            office VARCHAR(255),
            phone VARCHAR(255),
            fax VARCHAR(255),
            state VARCHAR(255) NOT NULL,
            district VARCHAR(255),
            at_large BOOLEAN,
            geoid INTEGER,
            missed_votes_pct NUMERIC(4, 2) NOT NULL,
            votes_with_party_pct NUMERIC(5, 2) NOT NULL,
            votes_against_party_pct NUMERIC(4, 2) NOT NULL
        );";
        let statement = Statement::from_string(
            manager.get_database_backend(),
            create_members_table_sql.to_owned(),
        );
        manager
            .get_connection()
            .execute(statement)
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_members_table_sql = "DROP TABLE members";
        let statement = Statement::from_string(
            manager.get_database_backend(),
            drop_members_table_sql.to_owned(),
        );

        manager
            .get_connection()
            .execute(statement)
            .await
            .map(|_| ())
    }
}
