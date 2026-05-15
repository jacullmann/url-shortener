# url-shortener

A minimal URL shortener. Rust backend (Axum) + Vue 3 frontend. URLs are stored in memory – no database required.

```
/
├── server/   Rust · Axum · DashMap
└── client/   Vue 3 · TypeScript · Tailwind v4 · Vite
```

---

## Getting started

### 1 – Server

```bash
cd server
cp .env.example .env   # adjust PORT and BASE_URL if needed
cargo run
# → http://localhost:3000
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

---

## API

### `POST /shorten`

Shorten a URL.

**Request**
```json
{ "url": "https://example.com/some/very/long/path" }
```

**Response `201`**
```json
{
  "id": "abc12345",
  "short_url": "http://localhost:3000/abc12345",
  "original_url": "https://example.com/some/very/long/path"
}
```

**Error `422`** – when the URL is invalid or uses an unsupported scheme:
```json
{ "error": "Invalid URL: not-a-url" }
```

### `GET /:id`

Redirects (`307 Temporary Redirect`) to the original URL.

**Error `404`** – when the ID is unknown:
```json
{ "error": "Url not found: abc12345" }
```

### `GET /health`

```json
{ "status": "ok" }
```

---

## Project structure

```
server/
├── src/
│   ├── lib.rs        Router, handlers, AppState, error types, tests
│   └── main.rs       Binary entrypoint – binds port, loads .env
├── Cargo.toml
└── .env.example

client/
├── src/
│   ├── api/
│   │   └── shortener.ts     Axios client typed to the API
│   ├── composables/
│   │   └── useShortener.ts  Business logic, VueUse clipboard
│   ├── components/
│   │   └── HistoryItem.vue  Single session-history row
│   ├── types/
│   │   └── api.ts           TypeScript interfaces matching Rust structs
│   ├── App.vue
│   ├── main.ts
│   └── style.css            Design tokens + reusable component classes
├── index.html
├── vite.config.ts
├── tsconfig.json
└── package.json
```

---

## Notes

- **No persistence.** All shortened URLs live in memory and are lost on server restart. This is intentional for simplicity.
- **No deduplication.** The same long URL gets a new ID on every `POST /shorten`.
- **Session history** in the frontend is in-memory only (a `ref` array). Refreshing the page clears it.

---

## Running tests (server)

```bash
cd server
cargo test
```