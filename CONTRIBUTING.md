# Contributing to Simulation Engine

Thank you for your interest in contributing to the Simulation Engine! We welcome contributions from the community.

## Development Setup

### Prerequisites
- Rust 1.90.0+ ([Install Rust](https://rustup.rs))
- Git

### Getting Started
```bash
# Clone the repository
git clone https://github.com/bniladridas/simulation-engine.git
cd simulation-engine

# Run tests and checks
./scripts/ci.sh

# Build the project
cargo build

# Run examples
cargo run --example basic_simulation
```

## Development Workflow

### 1. Choose an Issue
- Check [GitHub Issues](https://github.com/bniladridas/simulation-engine/issues) for open tasks
- Comment on the issue to indicate you're working on it

### 2. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 3. Make Changes
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Run `./scripts/ci.sh` to ensure quality

### 4. Commit Changes
- Use conventional commit format:
  ```bash
  feat: add new physics simulation feature
  fix: resolve collision detection bug
  docs: update API documentation
  refactor: improve code organization
  test: add unit tests for physics engine
  ```
- Keep commits focused and atomic

### 5. Submit Pull Request
- Push your branch to GitHub
- Create a pull request with a clear description
- Reference any related issues
- Ensure CI checks pass

## Code Guidelines

### Rust Style
- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` for linting
- Write comprehensive documentation comments
- Use meaningful variable and function names

### Testing
- Add unit tests for new functions
- Include integration tests for complex features
- Ensure all tests pass before submitting

### Documentation
- Update README.md for significant changes
- Add examples for new features
- Keep API documentation current

## Pull Request Title Format

Use the following format for pull request titles:

```
<type>[<scope>] :: <short, imperative description>
```

### Breakdown

* **`<type>`** — what kind of change this is
  Common values:
  * `feat` – new functionality
  * `fix` – bug or build fix
  * `refactor` – internal restructuring, no behavior change
  * `perf` – performance improvement
  * `docs` – documentation only
  * `chore` – tooling, CI, deps
  * `test` – tests only

* **`<scope>`** — affected area (keep it short, lowercase)
  Examples: `physics`, `engine`, `ci`, `ui`

* **`::`** — visual separator

* **`<short description>`**
  * Imperative mood ("add", "fix", "improve", "refactor")
  * ≤ 60–70 characters if possible
  * No period at the end

### Examples

* `feat[physics] :: improve numerical stability`
* `fix[engine] :: resolve collision detection bug`
* `refactor[ci] :: update pre-commit hooks`
* `chore[deps] :: upgrade rust dependencies`

## Commit Message Format

We use [Conventional Commits](https://conventionalcommits.org/):

```
<type>[optional scope]: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Testing changes
- `chore`: Maintenance tasks

## Reporting Issues

- Use [GitHub Issues](https://github.com/bniladridas/simulation-engine/issues) for bugs and feature requests
- Provide clear reproduction steps
- Include relevant code samples and error messages
- Specify your environment (OS, Rust version, etc.)

## License

By contributing to this project, you agree that your contributions will be licensed under the same MIT License that covers the project.
