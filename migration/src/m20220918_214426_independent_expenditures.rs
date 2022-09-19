use sea_orm_migration::prelude::*;
use super::m20220903_102025_members::Members;
use super::m20220918_161145_committees::Committees;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IndependentExpenditures::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(IndependentExpenditures::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(IndependentExpenditures::CommitteeId).string().not_null())
                    .col(ColumnDef::new(IndependentExpenditures::Amount).integer().not_null())
                    .col(ColumnDef::new(IndependentExpenditures::SupportOppose).string().not_null())
                    .col(ColumnDef::new(IndependentExpenditures::ElectionCycle).string().not_null())
                    .col(ColumnDef::new(IndependentExpenditures::CandidateId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-independent-expenditures-committees")
                            .from(IndependentExpenditures::Table, IndependentExpenditures::CommitteeId)
                            .to(Committees::Table, Committees::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-independent-expenditures-candidates")
                            .from(IndependentExpenditures::Table, IndependentExpenditures::CandidateId)
                            .to(Members::Table, Members::Id),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IndependentExpenditures::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum IndependentExpenditures {
    Id,
    Table,
    CommitteeId,
    Amount,
    SupportOppose,
    ElectionCycle,
    CandidateId
}
