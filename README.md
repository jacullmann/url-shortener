# url-shortener

Fast and minimal URL shortener. Rust backend and Vue.js client. Data is stored in memory via DashMap. No database required.

---

## Getting started

### 1 – Server

```bash
cd server
cp .env.example .env   # adjust PORT and BASE_URL if needed
cargo run
# → http://localhost:3000

cargo test # run tests for server
```

**Environment variables** (`server/.env`):

| Variable   | Default                    | Description                                 |
|------------|----------------------------|---------------------------------------------|
| `PORT`     | `3000`                     | Port the server listens on                  |
| `BASE_URL` | `http://localhost:{PORT}`  | Prefix used when constructing short URLs    |

### 2 – Client

```bash
cd client
pnpm install
pnpm run dev
# → http://localhost:5173
```

Vite proxies `/shorten` and `/health` to `http://localhost:3000` during development, so no CORS configuration is needed.

## Notes

- **No persistence.** All shortened URLs live in memory and are lost on server restart. This is intentional for simplicity.
- **No deduplication.** The same long URL gets a new ID on every `POST /shorten`.

---