use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let create_members_table_sql = "CREATE TABLE IF NOT EXISTS members (
            id VARCHAR(255) NOT NULL PRIMARY KEY,
            chamber VARCHAR(255),
            title VARCHAR(255),
            short_title VARCHAR(255),
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
            crp_id VARCHAR(255),
            google_entity_id VARCHAR(255),
            fec_candidate_id VARCHAR(255) NOT NULL,
            url VARCHAR(255) NOT NULL,
            rss_url VARCHAR(255),
            contact_form VARCHAR(255),
            in_office BOOLEAN NOT NULL,
            cook_pvi VARCHAR(255),
            dw_nominate NUMERIC(4, 3),
            ideal_point VARCHAR(255),
            seniority VARCHAR(255),
            next_election VARCHAR(255),
            total_votes INTEGER,
            missed_votes INTEGER,
            total_present INTEGER,
            last_updated VARCHAR(255) NOT NULL,
            ocd_id VARCHAR(255),
            office VARCHAR(255),
            phone VARCHAR(255),
            fax VARCHAR(255),
            state VARCHAR(255) NOT NULL,
            district VARCHAR(255),
            at_large BOOLEAN,
            geoid VARCHAR(255),
            missed_votes_pct NUMERIC,
            votes_with_party_pct NUMERIC,
            votes_against_party_pct NUMERIC
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
