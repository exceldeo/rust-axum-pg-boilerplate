# Rust Axum + SQLx Auth Boilerplate

A minimal, production-ready backend starter built with Axum 0.7, SQLx 0.7 (Postgres), Argon2 password hashing, and JWT authentication.

## Tech Stack

- Axum 0.7 (HTTP server, routing, middleware)
- SQLx 0.7 (Postgres, async, query macros)
- Tokio 1.x (async runtime)
- Argon2 0.5 (password hashing)
- JSON Web Tokens via `jsonwebtoken`
- CORS via `tower-http`
- Env management via `dotenvy`

## Features

- User registration with Argon2 password hashing
- Login with JWT issuance (access + refresh tokens)
- Modular routing and handlers
- Middleware skeleton for auth (Bearer token)
- Ready-to-use migration for a `users` table

## Project Structure

```
src/
  main.rs               # App bootstrap & server
  db/                   # Database setup
  handlers/             # Request handlers (auth, user)
  middleware/           # Auth extractor/middleware
  models/               # Data models & DTOs
  routes/               # Route composition (auth, etc.)
  services/             # Business logic (auth, token)
```

## Requirements

- Rust (stable) and Cargo
- Postgres database
- Optional: `sqlx-cli` for running migrations easily

Install `sqlx-cli` (recommended):

```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

## Environment Variables

Copy `.env.example` to `.env` and fill in values:

```
# Server
BIND_ADDRESS=127.0.0.1:3000

# Database
DATABASE_URL=postgres://USER:PASSWORD@localhost:5432/DB_NAME

# JWT
JWT_SECRET=super-secret-key-change-me
# Accepted formats: Xm (minutes), Xh (hours), Xd (days)
JWT_ACCESS_TOKEN_EXPIRY=15m
JWT_REFRESH_TOKEN_EXPIRY=7d
```

## Database & Migrations

Run migrations (using sqlx-cli):

```bash
# Ensure DATABASE_URL is set in your environment or .env
sqlx database create
sqlx migrate run
```

Alternatively, run with env inline:

```bash
DATABASE_URL=postgres://USER:PASSWORD@localhost:5432/DB_NAME sqlx migrate run
```

## Run the Server

```bash
cargo run
```

Server defaults to `127.0.0.1:3000`. You can change with `BIND_ADDRESS`.

### Hot Reload (Development)

Use `cargo-watch` to rebuild and restart on file changes:

```bash
cargo install cargo-watch
cargo watch -x run
```

## API Docs (Swagger / OpenAPI)

Interactive docs are available after the server starts:

- Swagger UI: `http://127.0.0.1:3000/docs`
- OpenAPI JSON: `http://127.0.0.1:3000/api-doc/openapi.json`

The docs include the `auth` endpoints (`/api/auth/register`, `/api/auth/login`).

## API Endpoints

Base path for auth routes is `/api/auth`.

- Register

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"Passw0rd!"}' \
  http://127.0.0.1:3000/api/auth/register
```

Response:

```json
{
  "status": "success",
  "message": "User registered successfully",
  "user_id": "<uuid>"
}
```

- Login

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -d '{"email":"alice@example.com","password":"Passw0rd!"}' \
  http://127.0.0.1:3000/api/auth/login
```

Response:

```json
{
  "status": "success",
  "access_token": "<jwt>",
  "refresh_token": "<jwt>"
}
```

- Example Protected Handler (skeleton)
  A sample handler exists at `handlers/user.rs` (`protected_handler`) and an auth extractor in `middleware/auth.rs`. To enable a protected route, add something like this in `main.rs`:

```rust
// inside `let app = Router::new()` builder
// .route("/api/protected", get(handlers::user::protected_handler))
```

Then call it with a Bearer token:

```bash
curl -H "Authorization: Bearer <access_token>" http://127.0.0.1:3000/api/protected
```

## Development Notes

- Uses Axum 0.7 server API: `axum::serve(listener, app.into_make_service())`.
- `JWT_ACCESS_TOKEN_EXPIRY` and `JWT_REFRESH_TOKEN_EXPIRY` accept `Xm`, `Xh`, or `Xd` formats.
- Token claims are defined in `models/token.rs` and reused in `services/token.rs`.

## Troubleshooting

- Build fails due to sqlx macros: ensure `DATABASE_URL` is set at build time if you use offline features, or avoid compile-time query validation.
- Connection errors: verify Postgres is running and `DATABASE_URL` is correct.
- 401 from protected routes: ensure you pass `Authorization: Bearer <access_token>` and `JWT_SECRET` matches the one used to sign tokens.

## License

MIT
