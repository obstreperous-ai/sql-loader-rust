use anyhow::{Context, Result};
use sqlx::{any::AnyPool, Pool};
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
pub async fn load_sql_file<P: AsRef<Path>>(
    database_url: &str,
    sql_file_path: P,
) -> Result<()> {
    // Connect to database
    let pool = connect_database(database_url)
        .await
        .context("Failed to connect to database")?;

    // Read SQL file
    let sql_content = fs::read_to_string(sql_file_path.as_ref())
        .with_context(|| {
            format!(
                "Failed to read SQL file: {}",
                sql_file_path.as_ref().display()
            )
        })?;

    // Execute SQL
    execute_sql(&pool, &sql_content)
        .await
        .context("Failed to execute SQL")?;

    Ok(())
}

/// Connect to a database using the provided URL
async fn connect_database(database_url: &str) -> Result<AnyPool> {
    let pool = AnyPool::connect(database_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", database_url))?;
    
    Ok(pool)
}

/// Execute SQL statements
async fn execute_sql(pool: &AnyPool, sql: &str) -> Result<()> {
    // Use raw_sql to execute the entire SQL script
    sqlx::raw_sql(sql)
        .execute(pool)
        .await
        .context("Failed to execute SQL statements")?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[tokio::test]
    async fn test_connect_database_sqlite() {
        let result = connect_database("sqlite::memory:").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_sql_basic() {
        let pool = AnyPool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect");

        let sql = "CREATE TABLE test (id INTEGER PRIMARY KEY);";
        let result = execute_sql(&pool, sql).await;
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
        temp_file.write_all(sql_content.as_bytes()).expect("Failed to write");
        
        // Test loading the file
        let result = load_sql_file("sqlite::memory:", temp_file.path()).await;
        assert!(result.is_ok());
    }
}
