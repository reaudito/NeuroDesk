// https://github.com/tursodatabase/turso/tree/main/bindings/rust

use anyhow::Result;
use turso::Builder;
use turso::Connection;

pub const DB_NAME: &str = "neurodesk.db";

pub async fn init_db() -> Result<Connection> {
    let db = Builder::new_local(DB_NAME).build().await?;
    let conn = db.connect()?;
    Ok(conn)
}
