# url-shortener

Fast and minimal URL shortener. Rust backend and Vue.js client. URLs are persisted in SQLite.

---

## Getting started

### 1 – Server

```bash
cd server
cp .env.example .env   # adjust variables as needed
cargo run
# → http://localhost:3000
```

**Run tests:**
```bash
cargo test
```

**Environment variables** (`server/.env`):

| Variable                | Default/Recommendation  | Description                                          |
|-------------------------|-------------------------|------------------------------------------------------|
| `PORT`                  | `3000`                  | Port the server listens on                           |
| `BASE_URL`              | `http://localhost:3000` | Prefix used when constructing short URLs             |
| `DATABASE_URL`          | `sqlite:urls.db`        | SQLite database path                                 |
| `RATE_LIMIT_PER_SECOND` | `1`                     | Token replenishment rate per IP (requests/second)    |
| `RATE_LIMIT_BURST`      | `60`                    | Maximum burst size per IP                            |

### 2 – Client

```bash
cd client
pnpm install
cp .env.example .env   # set BACKEND_URL if needed
pnpm run dev
# → http://localhost:5173
```

**Environment variables** (`client/.env`):

| Variable       | Default                              | Description                        |
|----------------|--------------------------------------|------------------------------------|
| `BACKEND_URL`  | `http://localhost:3000`              | URL of the running backend server  |


## Setup

The server uses `sqlx` compile-time query checking. The query cache is committed under `server/.sqlx/` so that `cargo build` works without a running database.

If you change SQL queries, regenerate the cache:

```bash
cd server
sqlx database create
sqlx migrate run
export DATABASE_URL=sqlite:urls.db
cargo sqlx prepare
git add .sqlx/
git commit -m "updated sqlx query cache"
```
