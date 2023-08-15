use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Name).string().not_null())
                    .col(ColumnDef::new(Post::Hash).blob(BlobSize::Blob(None)).null())
                    .col(ColumnDef::new(Post::Content).string().not_null())
                    .col(ColumnDef::new(Post::ParentPostId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-post-parent_post_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(Post::Table, Post::ParentPostId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Name,
    Hash,
    Content,
    ParentPostId,
}
