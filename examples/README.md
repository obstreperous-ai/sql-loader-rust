# Examples

This directory contains example SQL scripts demonstrating how to use sql-loader-rust.

## Running the Examples

### SQLite Example

```bash
# Create a SQLite database and load sample data
# Note: Use ?mode=rwc to create the database file if it doesn't exist
sql-loader --database sqlite://./example.db?mode=rwc --file examples/sample.sql

# Verify the data was loaded (using sqlite3 CLI)
sqlite3 example.db "SELECT * FROM users;"
```

### PostgreSQL Example

```bash
# Load data into a PostgreSQL database
sql-loader --database postgres://user:password@localhost:5432/mydb --file examples/sample.sql
```

## Creating Your Own SQL Scripts

Your SQL scripts can contain:

- DDL statements (CREATE TABLE, ALTER TABLE, etc.)
- DML statements (INSERT, UPDATE, DELETE)
- Multiple statements separated by semicolons
- Comments (using `--` for single-line or `/* */` for multi-line)

Example:

```sql
-- Create a table
CREATE TABLE employees (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    department TEXT
);

-- Insert data
INSERT INTO employees (id, name, department) VALUES 
    (1, 'Alice', 'Engineering'),
    (2, 'Bob', 'Sales');
```

## Tips

- Use `CREATE TABLE IF NOT EXISTS` for idempotent scripts
- Consider using transactions for large data loads
- Test your scripts on a development database first
- Keep scripts focused on a single logical unit of work
