use sea_orm::{ConnectionTrait, Schema};
use sea_orm::{Database, DbConn, EntityTrait};

use crate::Result;
use crate::{entity::*, Error};

pub async fn init(url: String) -> Result<DbConn> {
    let db = Database::connect(url).await?;
    migration(&db).await?;

    Ok(db)
}

async fn create_table<E>(db: &DbConn, entity: E) -> Result<()>
where
    E: EntityTrait,
{
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    db.execute(stmt)
        .await
        .map(|_| {
            log::info!("Migration table: {}", entity.table_name());
            ()
        })
        .map_err(|e| Error::from(e))
}

async fn migration(db: &DbConn) -> Result<()> {
    create_table(db, Category).await?;
    create_table(db, Post).await?;
    create_table(db, Tag).await?;
    create_table(db, PostTag).await?;

    log::info!("Migration view: cate_post_count");
    db.execute_unprepared(
        "
        CREATE OR REPLACE VIEW cate_post_count(id, name, count) AS
        SELECT c.id,
               c.name,
               count(p.id) AS count
        FROM category c
                LEFT JOIN post p ON c.id = p.category_id
        GROUP BY c.id;
        ",
    )
    .await?;

    log::info!("Migration view: tag_post_count");
    db.execute_unprepared(
        "
        CREATE OR REPLACE VIEW tag_post_count(id, name, count) AS
        SELECT t.id,
               t.name,
               count(pt.post_id) AS count
        FROM tag t
                LEFT JOIN post_tag pt ON t.id = pt.tag_id
        GROUP BY t.id;
        ",
    )
    .await?;

    Ok(())
}
