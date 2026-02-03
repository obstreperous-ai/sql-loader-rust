# sql-loader-rust

[![CI](https://github.com/obstreperous-ai/sql-loader-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/obstreperous-ai/sql-loader-rust/actions/workflows/ci.yml)
[![Release](https://github.com/obstreperous-ai/sql-loader-rust/actions/workflows/release.yml/badge.svg)](https://github.com/obstreperous-ai/sql-loader-rust/actions/workflows/release.yml)

A lean CLI utility in Rust to run SQL data load scripts in minimal environments. Built as a static binary, it's perfect for Kubernetes pods and containerized environments.

## Features

- 🚀 **Static Binary**: Single executable with no runtime dependencies
- 🗄️ **Multi-Database**: Support for PostgreSQL and SQLite
- ⚡ **Async**: Built on Tokio for efficient async I/O
- 🧪 **Test-First**: Comprehensive test coverage with embedded databases
- 📦 **Minimal**: Small binary size optimized for containers
- 🔒 **Production-Ready**: Built with Rust's safety guarantees

## Installation

### From Release

Download the latest release for your platform from the [releases page](https://github.com/obstreperous-ai/sql-loader-rust/releases).

```bash
# Linux (x86_64, statically linked)
curl -LO https://github.com/obstreperous-ai/sql-loader-rust/releases/latest/download/sql-loader-vX.Y.Z-x86_64-unknown-linux-musl.tar.gz
tar xzf sql-loader-vX.Y.Z-x86_64-unknown-linux-musl.tar.gz
chmod +x sql-loader
mv sql-loader /usr/local/bin/
```

### From Source

```bash
cargo install --path .
```

## Usage

```bash
# Load SQL script into PostgreSQL
sql-loader --database postgres://user:pass@localhost/dbname --file script.sql

# Load SQL script into SQLite (use ?mode=rwc to create file if it doesn't exist)
sql-loader --database sqlite://./data.db?mode=rwc --file migrations.sql

# Display help
sql-loader --help
```

### Command Line Options

- `--database <URI>` or `-d <URI>`: Database connection URI
  - PostgreSQL: `postgres://user:pass@host:port/dbname`
  - SQLite: `sqlite://path/to/database.db?mode=rwc` (use `?mode=rwc` to create file if needed)
  - In-memory SQLite: `sqlite::memory:`
- `--file <PATH>` or `-f <PATH>`: Path to SQL script file

## Development

### Prerequisites

- Rust (latest stable)
- Cargo

### Quick Start

```bash
# Clone the repository
git clone https://github.com/obstreperous-ai/sql-loader-rust.git
cd sql-loader-rust

# Build the project
cargo build

# Run tests
cargo test

# Run with sample arguments
cargo run -- --help
```

### Using Task Runner

This project uses [Task](https://taskfile.dev/) for common operations:

```bash
# Show available tasks
task

# Build the project
task build

# Run tests
task test

# Build static binary
task build-static

# Format and lint
task fmt
task lint

# Run all checks
task check
```

### Building Static Binary

For deployment in containers and Kubernetes:

```bash
# Add musl target
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target x86_64-unknown-linux-musl

# Binary will be at: target/x86_64-unknown-linux-musl/release/sql-loader
```

Or use the task runner:

```bash
task build-static
```

### Code Quality

This project maintains high code quality standards:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

### Development Container

Open in GitHub Codespaces or VS Code Dev Containers for a pre-configured development environment with Rust, Clippy, and all necessary tools.

## Testing

All tests use embedded databases (in-memory SQLite or embedded Postgres via sqlx) - no external dependencies required:

```bash
# Run all tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Docker Usage

```dockerfile
# Example Dockerfile
FROM scratch
COPY target/x86_64-unknown-linux-musl/release/sql-loader /sql-loader
ENTRYPOINT ["/sql-loader"]
```

## Contributing

1. Write tests first (test-driven development)
2. Ensure all tests pass: `cargo test`
3. Format code: `cargo fmt`
4. Lint code: `cargo clippy -- -D warnings`
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details

## Project Philosophy

This project follows these core principles:

- **One-task focus**: Keep the tool simple and focused on SQL data loading
- **Test-first development**: Write tests before implementation
- **Code quality**: Maintain high standards for production-grade Rust code
- **Minimalism**: Static binary with minimal dependencies
- **No external test dependencies**: All tests use embedded databases

For more details on development practices, see [.github/copilot-instructions.md](.github/copilot-instructions.md)
