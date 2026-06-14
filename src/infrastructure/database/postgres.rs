use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn setup_database() -> anyhow::Result<PgPool> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("DATABASE_URL not found in any source"))?;

    let pool = create_pool(&database_url).await?;
    run_migrations(&pool).await?;

    println!("Database ready");

    Ok(pool)
}

async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await;

    println!("Connected to PostgreSQL!");
    pool
}

async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
