<div align="center">
  <img src="https://avatars.githubusercontent.com/u/24594132?s=200&v=4" alt="Rust Crab Logo" width="200"/>
  <h1>Rust Axum REST API</h1>
</div>

A fun test project exploring Rust web development with the Axum framework. This is a learning playground for building REST APIs with modern Rust tooling.

## Features

- **Fast & Efficient**: Built with Rust and Axum for exceptional performance
- **JWT Authentication**: Secure token-based authentication system
- **Database Integration**: PostgreSQL with SQLx for type-safe database operations
- **Migration System**: Database schema management with SQLx migrations
- **Logging**: Structured logging with tracing and tracing-subscriber
- **Environment Configuration**: Environment variable management with dotenvy
- **Docker Support**: Containerized deployment with Docker Compose
- **Modern Tooling**: Justfile for common development tasks
- **Type Safety**: Leverages Rust's type system for compile-time guarantees

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Fast, ergonomic web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- **Authentication**: [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT token handling
- **Runtime**: [Tokio](https://tokio.rs/) - Async runtime
- **Serialization**: [Serde](https://serde.rs/) - Serialization framework
- **Logging**: [Tracing](https://tracing.rs/) - Application-level tracing
- **Environment**: [Dotenvy](https://github.com/allan2/dotenvy) - Environment variable loader
- **DateTime**: [Chrono](https://github.com/chronotope/chrono) - Date and time handling
- **Containerization**: Docker & Docker Compose

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (or Docker for containerized database)
- Docker & Docker Compose (optional, for full containerized development)

### Environment Setup

Create a `.env` file in the project root with the following variables:

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust-axum-rest-api
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
```

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rust-axum-rest-api.git
cd rust-axum-rest-api
```

2. Build the project:

```bash
just build
```

3. Set up the database (create, migrate, and seed):

```bash
just setup
```

4. Start the development server:

```bash
just dev
```

The API will be available at `http://localhost:5000`

### Using Docker

For containerized development:

```bash
docker-compose up --build
```

## Development

### Available Commands

**Development:**

- `just dev` - Start development server with database
- `just dev-watch` - Start development server with auto-reload
- `just generate-jwt <user_id>` - Generate JWT token for testing

**Building & Testing:**

- `just build` - Build the project
- `just build-release` - Build optimized release version
- `just check` - Check code without building
- `just test` - Run tests
- `just test-watch` - Run tests in watch mode

**Code Quality:**

- `just clippy` - Run Clippy linter
- `just fmt` - Format code
- `just fmt-check` - Check code formatting

**Database Management:**

- `just db-up` - Start PostgreSQL container
- `just db-down` - Stop PostgreSQL container
- `just db-reset` - Reset database container
- `just migrate` - Run database migrations
- `just migrate-revert` - Revert last migration
- `just seed` - Seed database with sample data
- `just setup` - Complete database setup (create + migrate + seed)

**Utilities:**

- `just clean` - Clean build artifacts and stop containers
- `just db-info` - Show database connection details

## API Endpoints

### Health Check

- `GET /` - Health check with database status

### Authentication

Most user and post endpoints require JWT authentication. Include the token in the Authorization header:

```
Authorization: Bearer <your-jwt-token>
```

### User Endpoints

- `GET /users` - Get all users
- `GET /users/{id}` - Get user by ID
- `POST /user` - Create new user
- `GET /user` - Get current user (requires auth)
- `PUT /user` - Update current user (requires auth)
- `DELETE /user` - Delete current user (requires auth)

### Post Endpoints

- `GET /posts` - Get all posts
- `GET /post/{id}` - Get post by ID
- `POST /post` - Create new post (requires auth)
- `PUT /post/{id}` - Update post (requires auth)
- `DELETE /post/{id}` - Delete post (requires auth)
- `GET /user/{id}/posts` - Get posts by user ID
- `GET /user/posts` - Get current user's posts (requires auth)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
- [SQLx](https://github.com/launchbadge/sqlx) for type-safe database operations
- The Rust community for the amazing ecosystem
