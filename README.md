# Foxtrot

A lightweight Rust REST API for user management, built with Axum, SQLx, and SQLite.

## Overview

Foxtrot is a small CRUD service that manages users (named `Person` in the model layer). The codebase follows a layered structure:

- handlers: HTTP request/response handling
- service: business rules and domain error mapping
- repository: SQLx data access
- models: request/response and DB row structs

The app runs on `0.0.0.0:3000` and persists data in a local SQLite database file named `test.db`.

## Project Structure

```text
src/
  main.rs
  db.rs
  error.rs
  models.rs
  handlers/
    mod.rs
    user_handlers.rs
  service/
    mod.rs
    user_service.rs
  repository/
    mod.rs
    user_repo.rs
migrations/
  20260709091325_create_users.sql
```

## Database

On startup, Foxtrot:

1. creates `test.db` if it does not exist
2. opens a SQLx SQLite pool
3. applies embedded migrations from `./migrations`

Current schema:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);
```

## API

### Routes

| Method | Path | Purpose |
|---|---|---|
| GET | `/` | Health check |
| GET | `/list` | List all users |
| POST | `/add_person` | Create a user |
| GET | `/person/{id}` | Get user by ID |
| PUT | `/update_person/{id}` | Update user name |
| DELETE | `/remove_person/{id}` | Delete user |

### Request Payload

`POST /add_person` and `PUT /update_person/{id}` expect:

```json
{
  "name": "Alice"
}
```

### Behavior and Status Codes

| Endpoint | Success | Not Found | Duplicate Name | Other Failure |
|---|---|---|---|---|
| GET `/list` | 200 | - | - | 500 |
| GET `/person/{id}` | 200 | 404 | - | 500 |
| POST `/add_person` | 201 | - | 409 | 500 |
| PUT `/update_person/{id}` | 200 | 202 | - | 500 |
| DELETE `/remove_person/{id}` | 204 | 404 | - | 500 |

Notes:

- `PUT /update_person/{id}` currently returns `202 Accepted` when the user does not exist.
- `DELETE /remove_person/{id}` returns `204 No Content` and attempts to include a text body, though clients may ignore response bodies for 204.

## Quick Start

### Prerequisites

- Rust toolchain (edition 2024)

### Run

```bash
cargo run
```

Server address:

```text
http://127.0.0.1:3000
```

## Example Requests

Create user:

```bash
curl -X POST http://127.0.0.1:3000/add_person \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice"}'
```

List users:

```bash
curl http://127.0.0.1:3000/list
```

Get user by ID:

```bash
curl http://127.0.0.1:3000/person/1
```

Update user:

```bash
curl -X PUT http://127.0.0.1:3000/update_person/1 \
  -H "Content-Type: application/json" \
  -d '{"name":"Bob"}'
```

Delete user:

```bash
curl -X DELETE http://127.0.0.1:3000/remove_person/1
```

## Dependencies

- tokio
- axum
- sqlx (sqlite, runtime-tokio, tls-native-tls, migrate)
- serde
- serde_json
- thiserror
- anyhow
