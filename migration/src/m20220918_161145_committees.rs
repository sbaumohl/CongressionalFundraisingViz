use sea_orm_migration::prelude::*;
use super::m20220903_102025_members::Members;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Committees::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Committees::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Committees::Name).string().not_null())
                    .col(ColumnDef::new(Committees::Designation).string().not_null())
                    .col(ColumnDef::new(Committees::OrgType).string().not_null())
                    .col(ColumnDef::new(Committees::ConnectedOrg).string().not_null())
                    .col(ColumnDef::new(Committees::CandidateId).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-committees-candidates")
                            .from(Committees::Table, Committees::CandidateId)
                            .to(Members::Table, Members::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Committees::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Committees {
    Table,
    Id,
    Name,
    Designation,
    OrgType,
    ConnectedOrg,
    CandidateId
}
