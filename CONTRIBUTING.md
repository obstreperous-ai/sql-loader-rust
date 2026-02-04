# Contributing to sql-loader-rust

Thank you for your interest in contributing to sql-loader-rust! This document provides guidelines and instructions for contributing to the project.

## Development Philosophy

This project follows these core principles:

1. **Test-First Development**: Write tests before implementing features
2. **Code Quality**: Maintain high standards for production-grade Rust code
3. **One-Task Focus**: Keep the tool focused on SQL data loading
4. **Minimalism**: Prefer simplicity and small binary size

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Git
- (Optional) [Task](https://taskfile.dev/) for running common tasks

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/sql-loader-rust.git
   cd sql-loader-rust
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## Development Workflow

### 1. Write Tests First

Before implementing any feature:

1. Write a failing test that describes the expected behavior
2. Run `cargo test` to confirm the test fails
3. Implement the minimum code to make the test pass
4. Refactor as needed while keeping tests green

Example:
```rust
#[tokio::test]
async fn test_new_feature() {
    // Arrange
    let input = "test data";
    
    // Act
    let result = new_feature(input).await;
    
    // Assert
    assert!(result.is_ok());
}
```

### 2. Code Quality Standards

Before submitting a PR, ensure your code passes all quality checks:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test
```

Or use the task runner:
```bash
task check
```

### 3. Testing Requirements

- All tests must use embedded databases (in-memory SQLite or embedded Postgres)
- **Never** use Docker containers or external database instances
- Tests should be fast and deterministic
- Aim for high code coverage

### 4. Commit Messages

Write clear, descriptive commit messages:

```
Short summary (50 chars or less)

More detailed explanation if needed. Wrap at 72 characters.
Explain what changed and why, not how.

- Bullet points are okay
- Reference issues: #123
```

### 5. Pull Request Process

1. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the guidelines above

3. Ensure all tests pass and code is formatted:
   ```bash
   task check
   ```

4. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

5. Create a Pull Request with:
   - Clear description of changes
   - Link to related issues
   - Test results

6. Wait for code review and address feedback

## Code Style

- Follow Rust idioms and best practices
- Use meaningful variable and function names
- Write documentation comments (`///`) for public APIs
- Keep functions focused and small
- Prefer composition over complexity
- Handle errors properly using `Result` types
- Avoid `unwrap()` in production code

## Documentation

- Update README.md if adding user-facing features
- Add doc comments for public APIs
- Include examples in documentation where helpful
- Update CHANGELOG.md (if exists) with notable changes

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // test code
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use sql_loader_rust::*;

#[tokio::test]
async fn test_integration() {
    // test code
}
```

### Database Testing

Always use embedded databases:

```rust
// Good: In-memory SQLite
let pool = SqlitePool::connect("sqlite::memory:").await?;

// Bad: External database
let pool = SqlitePool::connect("sqlite://external.db").await?;
```

## Questions?

If you have questions or need help:

1. Check existing issues
2. Review documentation
3. Open a new issue with your question

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
