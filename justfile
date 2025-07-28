# Default recipe to run when no arguments are provided
default:
    @just --list

# Database management
db-up:
    docker-compose up -d postgres

db-down:
    docker-compose down

db-logs:
    docker-compose logs postgres

db-reset:
    docker-compose down -v
    docker-compose up -d postgres

# Cargo commands
build:
    cargo build

build-release:
    cargo build --release

check:
    cargo check

test:
    cargo test

test-watch:
    cargo test --watch

clippy:
    cargo clippy

fmt:
    cargo fmt

fmt-check:
    cargo fmt -- --check

# Development workflow
dev: db-up
    cargo run

generate-jwt user_id:
    cargo run --example generate_token {{user_id}}

dev-watch: db-up
    cargo watch -x run

clean:
    cargo clean
    docker-compose down

# Database connection info
db-info:
    @echo "Database connection:"
    @echo "  Host: localhost"
    @echo "  Port: 5432"
    @echo "  Database: rust-axum-rest-api"
    @echo "  User: postgres"
    @echo "  Password: password"

# SQLx database management
db-create: db-up
    sqlx database create

db-drop:
    sqlx database drop

db-reset-sqlx: db-down
    sqlx database drop
    sqlx database create

# SQLx migrations
migrate: db-up
    sqlx migrate run

migrate-revert:
    sqlx migrate revert

migrate-info:
    sqlx migrate info

migrate-add migration_name:
    sqlx migrate add {{migration_name}}

# SQLx query testing
query:
    sqlx query

# Database seeding
seed:
    @echo "Running SQL seed files..."
    @psql postgresql://postgres:password@localhost:5432/rust-axum-rest-api -f seeds/01_users.sql
    @psql postgresql://postgres:password@localhost:5432/rust-axum-rest-api -f seeds/02_posts.sql

# Full database setup (create + migrate + seed)
setup: db-up
    sqlx database create
    sqlx migrate run
    just seed 