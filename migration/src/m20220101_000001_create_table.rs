use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(EsAggregates::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EsAggregates::Id)
                        .uuid()
                        .not_null()
                        .primary_key())
                    .col(ColumnDef::new(EsAggregates::Version)
                            .integer()
                            .not_null())
                    .col(ColumnDef::new(EsAggregates::AggregateType)
                        .string()
                        .not_null())
                    .to_owned(),
            ).await?;

            manager.create_index(
                Index::create()
                    .if_not_exists()
                    .table(EsAggregates::Table)
                    .name("idx_es_aggregates_aggregate_type")
                    .col(EsAggregates::AggregateType)
                    .to_owned(),
            ).await?;

        manager
            .create_table(
                Table::create()
                    .table(EsEvents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EsEvents::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key())
                    .col(ColumnDef::new(EsEvents::AggregateId)
                        .uuid()
                        .not_null())
                    .col(ColumnDef::new(EsEvents::Version)
                        .integer()
                        .not_null())
                    .col(ColumnDef::new(EsEvents::EventType)
                        .text()
                        .not_null())
                    .col(ColumnDef::new(EsEvents::JsonData)
                        .json()
                        .not_null())
                    .col(ColumnDef::new(EsEvents::MetaData)
                        .json()
                        .not_null())
                    .col(ColumnDef::new(EsEvents::CreateAt)
                        .timestamp()
                        .not_null()
                        .default(Expr::current_timestamp()))
                    .to_owned()
            ).await?;

            manager.create_foreign_key(ForeignKey::create()
                .name("fk_events_aggregate")
                .from(EsEvents::Table, EsEvents::AggregateId)
                .to(EsAggregates::Table, EsAggregates::Id)
                .to_owned()
            ).await?;

            manager.create_index(Index::create()
                .if_not_exists()
                .table(EsEvents::Table)
                .name("idx_es_events_version")
                .col(EsEvents::Version).to_owned()
            ).await?;

            manager.create_index(Index::create()
                .if_not_exists()
                .table(EsEvents::Table)
                .name("idx_es_events_aggregate_id")
                .col(EsEvents::AggregateId)
                .to_owned()
            ).await?;
            
            manager.create_index(Index::create()
                .unique()
                .table(EsEvents::Table)
                .name("idx_aggregate_ids_versions")
                .col(EsEvents::AggregateId)
                .col(EsEvents::Version).to_owned()
            ).await?;

            manager
            .create_table(
                Table::create()
                    .table(EsAggregateSnapshots::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(EsAggregateSnapshots::AggregateId)
                        .uuid()
                        .not_null()
                        .primary_key())
                    .col(ColumnDef::new(EsAggregateSnapshots::Version)
                            .integer()
                            .not_null())
                    .col(json(EsAggregateSnapshots::JsonData).not_null())
                    .col(ColumnDef::new(EsAggregateSnapshots::Revision)
                            .uuid()
                            .not_null())
                    .to_owned(),
            ).await?;

            manager.create_foreign_key(ForeignKey::create()
                .name("fk_aggregate_snapshots_events")
                .from(EsAggregateSnapshots::Table, EsAggregateSnapshots::AggregateId)
                .to(EsAggregates::Table, EsAggregates::Id)
                .to_owned()
            ).await?;
            
            Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.drop_table(Table::drop().table(EsAggregateSnapshots::Table).to_owned())
            .await?;
        manager.drop_table(Table::drop().table(EsEvents::Table).to_owned())
            .await?;
        manager.drop_table(Table::drop().table(EsAggregates::Table).to_owned())
            .await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
enum EsAggregates {
    Table,
    Id,
    Version,
    AggregateType,
}

#[derive(DeriveIden)]
enum EsEvents {
    Table,
    Id,
    AggregateId,
    Version,
    EventType,
    JsonData,
    MetaData,
    CreateAt
}

#[derive(DeriveIden)]
enum EsAggregateSnapshots {
    Table,
    AggregateId,
    Version,
    JsonData,
    Revision,
}