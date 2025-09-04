use sqlx::{migrate::Migrator, PgPool};
use tokio::sync::OnceCell;
use std::path::Path;

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init(url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPool::connect(url).await?;
    POOL.set(pool).unwrap();
    Ok(())
}

pub async fn migrate() -> Result<(), sqlx::Error> {
    // Create a temporary pool just for migration
    let pool = get_pool();
    let migrations_path = Path::new("./migrations");
    let migrator = Migrator::new(migrations_path).await?;
    migrator.run(pool).await?;
    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    POOL.get().expect("Database not initialized")
}