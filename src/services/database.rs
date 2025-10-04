use sqlx::migrate::Migrator;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::path::PathBuf;

static MIGRATOR: Migrator = sqlx::migrate!();

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    MIGRATOR.run(pool).await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct DatabaseService {
    pub pool: SqlitePool,
}

impl DatabaseService {
    pub async fn new(path: PathBuf) -> Self {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create database directory");
        }

        let url = format!("sqlite://{}?mode=rwc", path.display());

        let pool = SqlitePoolOptions::new()
            .connect(&url)
            .await
            .expect("Cannot open or create the database file");

        run_migrations(&pool)
            .await
            .expect("could not run database migrations");

        Self { pool }
    }
}
