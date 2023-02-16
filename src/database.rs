use crate::Result;
use sea_orm::{Database, DbConn};

pub async fn init(url: String) -> Result<DbConn> {
    let db = Database::connect(url).await?;

    Ok(db)
}
