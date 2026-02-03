use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::path::Path;

/// Load and execute SQL from a file
///
/// # Arguments
///
/// * `database_url` - Database connection URL (postgres:// or sqlite://)
/// * `sql_file_path` - Path to the SQL file to execute
///
/// # Examples
///
/// ```no_run
/// use sql_loader_rust::load_sql_file;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     load_sql_file("sqlite::memory:", "migrations.sql").await?;
///     Ok(())
/// }
/// ```
pub async fn load_sql_file<P: AsRef<Path>>(database_url: &str, sql_file_path: P) -> Result<()> {
    // Read SQL file
    let sql_content = fs::read_to_string(sql_file_path.as_ref()).with_context(|| {
        format!(
            "Failed to read SQL file: {}",
            sql_file_path.as_ref().display()
        )
    })?;

    // Determine database type and execute
    if database_url.starts_with("sqlite:") {
        let pool = sqlx::SqlitePool::connect(database_url)
            .await
            .with_context(|| format!("Failed to connect to database: {}", database_url))?;

        execute_sql_sqlite(&pool, &sql_content)
            .await
            .context("Failed to execute SQL")?;
    } else if database_url.starts_with("postgres:") {
        let pool = sqlx::PgPool::connect(database_url)
            .await
            .with_context(|| format!("Failed to connect to database: {}", database_url))?;

        execute_sql_postgres(&pool, &sql_content)
            .await
            .context("Failed to execute SQL")?;
    } else {
        anyhow::bail!("Unsupported database URL scheme. Use 'sqlite:' or 'postgres:'");
    }

    Ok(())
}

/// Execute SQL statements for SQLite
async fn execute_sql_sqlite(pool: &Pool<Sqlite>, sql: &str) -> Result<()> {
    sqlx::raw_sql(sql)
        .execute(pool)
        .await
        .context("Failed to execute SQL statements")?;

    Ok(())
}

/// Execute SQL statements for Postgres
async fn execute_sql_postgres(pool: &sqlx::PgPool, sql: &str) -> Result<()> {
    sqlx::raw_sql(sql)
        .execute(pool)
        .await
        .context("Failed to execute SQL statements")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_execute_sql_sqlite_basic() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect");

        let sql = "CREATE TABLE test (id INTEGER PRIMARY KEY);";
        let result = execute_sql_sqlite(&pool, sql).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_load_sql_file_integration() {
        // Create a temporary SQL file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let sql_content = r#"
            CREATE TABLE test_table (
                id INTEGER PRIMARY KEY,
                value TEXT
            );
            INSERT INTO test_table (id, value) VALUES (1, 'test');
        "#;
        temp_file
            .write_all(sql_content.as_bytes())
            .expect("Failed to write");

        // Test loading the file
        let result = load_sql_file("sqlite::memory:", temp_file.path()).await;
        assert!(result.is_ok());
    }
}
