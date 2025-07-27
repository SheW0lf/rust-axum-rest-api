<div align="center">
  <img src="https://avatars.githubusercontent.com/u/24594132?s=200&v=4" alt="Rust Crab Logo" width="200"/>
  <h1>Rust Axum REST API</h1>
</div>

A fun test project exploring Rust web development with the Axum framework. This is a learning playground for building REST APIs with modern Rust tooling.

## Features

- **Fast & Efficient**: Built with Rust and Axum for exceptional performance
- **Database Integration**: PostgreSQL with SQLx for type-safe database operations
- **Migration System**: Database schema management with SQLx migrations
- **Docker Support**: Containerized deployment with Docker Compose
- **Modern Tooling**: Justfile for common development tasks
- **Type Safety**: Leverages Rust's type system for compile-time guarantees

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Fast, ergonomic web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- **Runtime**: [Tokio](https://tokio.rs/) - Async runtime
- **Serialization**: [Serde](https://serde.rs/) - Serialization framework
- **Containerization**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker & Docker Compose (optional, for containerized development)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-axum-rest-api.git
cd rust-axum-rest-api
```

2. Install dependencies:

```bash
just build
```

3. Run database migrations:

```bash
just migrate
```

4. Start the development server:

```bash
just run
```

The API will be available at `http://localhost:3000`

### Using Docker

For containerized development:

```bash
docker-compose up --build
```

## Development

### Available Commands

- `just run` - Start the development server
- `just test` - Run tests
- `just check` - Check code without building
- `just build` - Build the project
- `just migrate` - Run database migrations
- `just migrate-revert` - Revert last migration

## API Endpoints

_Documentation for API endpoints will be added as the project develops_

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
- [SQLx](https://github.com/launchbadge/sqlx) for type-safe database operations
- The Rust community for the amazing ecosystem
