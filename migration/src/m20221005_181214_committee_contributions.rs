use sea_orm_migration::prelude::*;
use super::{m20220903_102025_members::Members, m20220918_161145_committees::Committees};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommitteeContributions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CommitteeContributions::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(CommitteeContributions::SpenderCommittee).string().not_null())
                    .col(ColumnDef::new(CommitteeContributions::ElectionCycle).string().not_null())
                    .col(ColumnDef::new(CommitteeContributions::RecipientCommittee).string().not_null())
                    .col(ColumnDef::new(CommitteeContributions::RecipientCandidate).string().not_null())
                    .col(ColumnDef::new(CommitteeContributions::Amount).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-committee-contributions-spender-committee")
                            .from(CommitteeContributions::Table, CommitteeContributions::SpenderCommittee)
                            .to(Committees::Table, Committees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-committee-contributions-recipient-committee")
                            .from(CommitteeContributions::Table, CommitteeContributions::RecipientCommittee)
                            .to(Committees::Table, Committees::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-committee-contributions-recipient-candidate")
                            .from(CommitteeContributions::Table, CommitteeContributions::RecipientCandidate)
                            .to(Members::Table, Members::FecCandidateId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommitteeContributions::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CommitteeContributions {
    Table,
    Id,
    SpenderCommittee,
    RecipientCommittee,
    RecipientCandidate,
    ElectionCycle,
    Amount
}
