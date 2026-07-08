# Foxtrot

A lightweight REST API for user management built with [Axum](https://github.com/tokio-rs/axum), [SQLx](https://github.com/launchbadge/sqlx), and [SQLite](https://www.sqlite.org/).

## Overview

Foxtrot is a Rust web server that provides CRUD operations for managing users (referred to as "persons"). It follows a layered architecture separating handlers, services, and repository logic.

## Architecture

The project is organized into four layers:

```
src/
├── main.rs              # Application entry point & router setup
├── db.rs                # Database initialization (SQLite)
├── models.rs            # Data structures (Person, UserRequest)
├── error.rs             # Custom error types & HTTP response mapping
├── handlers/            # HTTP request handlers
│   ├── mod.rs
│   └── user_handlers.rs
├── service/             # Business logic layer
│   ├── mod.rs
│   └── user_service.rs
└── repository/          # Data access layer (SQL queries)
    ├── mod.rs
    └── user_repo.rs
```

### Layers

| Layer | Responsibility |
|---|---|
| **Handlers** | Parse HTTP requests, delegate to services, return HTTP responses |
| **Service** | Business logic, error mapping, orchestration |
| **Repository** | Raw SQL queries via SQLx, data access |
| **Models** | Shared data structures and request/response types |

## API Endpoints

| Method | Route | Description |
|---|---|---|
| `GET` | `/` | Health check — returns `"hello"` |
| `GET` | `/list` | List all users |
| `POST` | `/add_person` | Create a new user |
| `GET` | `/person/{id}` | Get a user by ID |
| `PUT` | `/update_person/{id}` | Update a user's name |
| `DELETE` | `/remove_person/{id}` | Delete a user |

### Request / Response Examples

**Create a user**

```bash
curl -X POST http://localhost:3000/add_person \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice"}'
```

**List all users**

```bash
curl http://localhost:3000/list
```

**Get a user by ID**

```bash
curl http://localhost:3000/person/1
```

**Update a user**

```bash
curl -X PUT http://localhost:3000/update_person/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Bob"}'
```

**Delete a user**

```bash
curl -X DELETE http://localhost:3000/remove_person/1
```

## Data Model

```rust
struct Person {
    id: i32,       // Auto-incremented primary key
    name: String,  // Unique user name
}
```

The underlying SQLite table `users` is created automatically on startup with columns `id` (INTEGER PRIMARY KEY AUTOINCREMENT) and `name` (TEXT NOT NULL UNIQUE).

## Error Handling

Custom errors are defined in [`src/error.rs`](src/error.rs) and mapped to appropriate HTTP status codes:

| Error | HTTP Status |
|---|---|
| `NotFound` | `404 Not Found` |
| `Conflict` (duplicate name) | `409 Conflict` |
| `Database` | `500 Internal Server Error` |

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (edition 2024)
- SQLite (included via SQLx, no separate installation required)

### Run the server

```bash
cargo run
```

The server starts on `http://0.0.0.0:3000`. A SQLite database file (`test.db`) is created automatically in the project root.

## Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [tokio](https://tokio.rs/) | 1.42.0 | Async runtime |
| [axum](https://github.com/tokio-rs/axum) | 0.8.1 | Web framework |
| [sqlx](https://github.com/launchbadge/sqlx) | 0.8.3 | SQLite driver (async) |
| [serde](https://serde.rs/) | 1.0.215 | Serialization/deserialization |
| [serde_json](https://github.com/serde-rs/json) | 1.0.134 | JSON support |
| [thiserror](https://github.com/dtolnay/thiserror) | 2.0.18 | Error derive macros |
| [anyhow](https://github.com/dtolnay/anyhow) | 1.0.97 | Error handling |
