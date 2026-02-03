# Copilot Instructions for sql-loader-rust

## Project Overview

This is a lean CLI utility in Rust to run SQL data load scripts in a minimal environment (static binary for K8s pods). The project emphasizes:
- **One-task focus**: Keep the tool simple and focused on SQL data loading
- **Test-first development**: Write tests before implementation
- **Code quality**: Maintain high standards for production-grade Rust code
- **Minimalism**: Static binary with minimal dependencies

## Tech Stack

- **Language**: Rust (latest stable)
- **SQL Libraries**: 
  - Use `sqlx` or `rusqlite` for database interactions
  - Support both PostgreSQL and SQLite
- **Testing**: 
  - Use embedded databases (in-memory SQLite or embedded Postgres via `sqlx`)
  - **Never use separate containers or external database instances for tests**
  - All tests must be self-contained and runnable without external dependencies
- **Build**: Produce a static binary suitable for K8s pods

## Development Practices

### Test-First Approach
- Always write tests before implementing new features
- Each feature should have corresponding unit and integration tests
- Tests must use embedded/in-memory databases (e.g., `sqlite::memory:` or embedded Postgres)
- Tests should be fast and deterministic

### Code Quality Standards
- Follow Rust best practices and idioms
- Use `clippy` for linting: run `cargo clippy -- -D warnings`
- Format code with `rustfmt`: run `cargo fmt`
- Write clear, self-documenting code with appropriate comments for complex logic
- Handle errors properly using `Result` types, avoid `unwrap()` in production code
- Write documentation comments (`///`) for public APIs

### One-Task Focus
- Keep the CLI focused on SQL data loading only
- Avoid feature creep or unrelated functionality
- Each module should have a single, well-defined responsibility
- Prefer composition over complexity

## Build and Test Commands

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Build the project
cargo build

# Build release (static binary)
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test <test_name>
```

## Restrictions and Guidelines

### Database Testing
- **MUST**: Use embedded databases for all tests (in-memory SQLite or embedded Postgres via sqlx)
- **MUST NOT**: Use Docker containers or external database instances for tests
- **MUST NOT**: Require any external services to run tests

### Code Constraints
- Minimize dependencies to reduce binary size
- Prefer standard library when possible
- Use well-maintained crates from crates.io
- Validate all user inputs
- Never commit secrets or credentials
- Never use `unsafe` code without thorough justification and documentation

### Static Binary Requirements
- Target `x86_64-unknown-linux-musl` or similar for static linking
- Keep binary size minimal (avoid unnecessary dependencies)
- Ensure the binary is self-contained with no external runtime dependencies

## Error Handling

- Use idiomatic Rust error handling with `Result<T, E>` and `?` operator
- Consider using `anyhow` or `thiserror` for error handling
- Provide clear, actionable error messages for users
- Log errors appropriately for debugging

## Documentation

- Document all public APIs with doc comments (`///`)
- Include examples in documentation where helpful
- Keep README.md updated with usage instructions
- Document configuration options and environment variables

## Project Structure

```
sql-loader-rust/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library code
│   ├── loader/           # SQL loading logic
│   ├── parser/           # SQL script parsing
│   └── db/               # Database connection and execution
├── tests/                # Integration tests
├── Cargo.toml            # Dependencies and metadata
└── README.md             # User documentation
```

## Context for AI Agents

This repository is for a production-grade Rust CLI tool. When working on tasks:
- Prioritize correctness and reliability over speed of implementation
- Write tests first to validate behavior
- Keep changes minimal and focused
- Follow Rust idioms and best practices
- Ensure all tests pass before completing work
- Use embedded databases for testing - never external services
