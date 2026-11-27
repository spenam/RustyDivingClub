# Rusty Diving Club Manager 

A Rust rewrite of the original locally hosted diving-club equipment and loan manager.

## Stack

- Rust stable, Tokio and Axum
- PostgreSQL with SQLx and checked-in SQL migrations
- Serde request/response models and `validator`
- Docker Compose for local deployment

## Run with Docker

```bash
cp .env.example .env
# Set a strong POSTGRES_PASSWORD in your shell or compose environment.
docker compose up --build
```

Open http://localhost:8080. The dashboard is at `/`; health is `/health`.

## Run locally

Install Rust and PostgreSQL, create a database and set `DATABASE_URL`:

```bash
cp .env.example .env
cargo run
```

Migrations run automatically at startup.

## API

| Method | Path | Purpose |
|---|---|---|
| GET | `/health` | Health check |
| GET | `/` | Dashboard |
| GET / POST | `/api/equipment` | List / register equipment |
| GET | `/api/equipment/:id` | Equipment details |
| GET / POST | `/api/members` | List / register members |
| GET | `/api/loans` | Loan history |
| POST | `/api/loans` | Check out equipment |
| POST | `/api/loans/:id/return` | Return equipment |

Example equipment:

```json
{"name":"Regulator set","description":"Primary regulator"}
```

Example member:

```json
{"full_name":"Alex Example","email":"alex@example.org"}
```

Example checkout:

```json
{"equipment_id":"EQUIPMENT_UUID","member_id":"MEMBER_UUID"}
```

Equipment QR payloads use `DCM:<qr_uuid>`. The UUID is generated independently of the equipment's database ID and remains stable if the display name changes.

## Safety and operational notes

- Checkout is transactional and locks the equipment row; only `available` equipment can be loaned.
- A partial unique index prevents multiple open loans for one item.
- Maintenance and unavailable states are not eligible for checkout.
- The starter intentionally does not expose PostgreSQL to the host.
- Authentication, role-based permissions, audit logs, incident management, inspection/clearance workflow, camera scanning, backups and HTTPS/VPN are production follow-ups. Do not expose this starter publicly before implementing them.
- Never commit `.env`, member personal data, or database backups.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```
