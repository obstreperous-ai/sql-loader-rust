use sqlx::sqlite::SqlitePool;
use std::fs;
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_sqlite_connection() {
    // Create an in-memory SQLite database
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory SQLite");

    // Verify connection by creating a table
    sqlx::query("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&pool)
        .await
        .expect("Failed to create test table");

    // Insert a row
    sqlx::query("INSERT INTO test (id, name) VALUES (1, 'test_value')")
        .execute(&pool)
        .await
        .expect("Failed to insert test data");

    // Query the row
    let result: (i32, String) = sqlx::query_as("SELECT id, name FROM test WHERE id = 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to query test data");

    assert_eq!(result.0, 1);
    assert_eq!(result.1, "test_value");
}

#[tokio::test]
async fn test_execute_sql_file() {
    // Create a temporary SQL file
    let sql_content = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        );
        
        INSERT INTO users (id, username, email) VALUES (1, 'alice', 'alice@example.com');
        INSERT INTO users (id, username, email) VALUES (2, 'bob', 'bob@example.com');
    "#;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    use std::io::Write;
    temp_file
        .write_all(sql_content.as_bytes())
        .expect("Failed to write to temp file");

    // Create in-memory database
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database");

    // Read and execute SQL file
    let sql_path = temp_file.path();
    let sql = fs::read_to_string(sql_path).expect("Failed to read SQL file");

    // Execute SQL statements
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("Failed to execute SQL");

    // Verify data was inserted
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .expect("Failed to count users");

    assert_eq!(count.0, 2);

    // Verify specific user
    let user: (String, String) = sqlx::query_as("SELECT username, email FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to fetch user");

    assert_eq!(user.0, "alice");
    assert_eq!(user.1, "alice@example.com");
}

#[tokio::test]
async fn test_multiple_statements() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database");

    let sql = r#"
        CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT);
        INSERT INTO products (id, name) VALUES (1, 'Product A');
        INSERT INTO products (id, name) VALUES (2, 'Product B');
    "#;

    // Execute multiple statements
    sqlx::raw_sql(sql)
        .execute(&pool)
        .await
        .expect("Failed to execute multiple statements");

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await
        .expect("Failed to count products");

    assert_eq!(count.0, 2);
}
