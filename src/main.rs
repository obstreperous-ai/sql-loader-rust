use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "sql-loader",
    about = "A lean CLI utility to run SQL data load scripts in minimal environments",
    version
)]
struct Args {
    /// Database connection URL (e.g., postgres://user:pass@host/db or sqlite://path/to/db.db)
    #[arg(short, long, env = "DATABASE_URL")]
    database: String,

    /// Path to SQL file to execute
    #[arg(short, long)]
    file: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Database: {}", args.database);
        println!("SQL File: {}", args.file.display());
    }

    // Load and execute SQL file
    sql_loader_rust::load_sql_file(&args.database, &args.file).await?;

    if args.verbose {
        println!("SQL file executed successfully!");
    }

    Ok(())
}
