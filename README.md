# Rusty Diving Club Manager

A local diving-club management app built with Rust, Axum, PostgreSQL and a browser dashboard.

## Included

- Member directory and diving levels `N1`, `PN1`, `N2`, `PN2`, `N3`, `PN3`, `N4`, `PN4`, `E1`, `E2`, `E3`, `E4`.
- Equipment categories and separately tracked physical items, serial numbers and stable QR UUIDs.
- Optional equipment brand for all items; regulator connection type (`DIN` / `ETRIER`); size for BCDs and wetsuits.
- Independent equipment borrowing status (`available` / `checked_out`) and condition (`ready` / `needs_repair`).
- Multi-item checkout for one member in a single transaction; each physical item has its own loan and shared `loan_group_id`.
- Optional due dates, overdue filtering and individual returns.
- Return inspection with a comment and an explicit condition selection.
- Append-only event timeline for item registration, checkout, return, maintenance/condition changes and notes.
- Row locking and a partial unique database index prevent double checkout; damaged items cannot be borrowed.

## Run on Windows with Docker Desktop

Open Docker Desktop, then PowerShell in the extracted project directory:

```powershell
docker compose up --build -d
docker compose ps
docker compose logs -f app
```

Open **http://localhost:8080**. Health endpoint: **http://localhost:8080/health**. SQL migrations run automatically when the app starts.

Stop without deleting your database:

```powershell
docker compose down
```

Restart:

```powershell
docker compose up -d
```

Rebuild after source changes:

```powershell
docker compose up --build -d
docker compose logs -f app
```

**Do not use `docker compose down -v` unless you intend to erase all local club data.** Back up the PostgreSQL volume before production upgrades.

## PowerShell API examples

Use `$null` (not bare `null`) and `Invoke-RestMethod` for JSON.

### Add a category

```powershell
$body = @{ name = "Regulators"; description = "Club regulator sets" } | ConvertTo-Json -Compress
Invoke-RestMethod -Uri "http://localhost:8080/api/categories" -Method Post -ContentType "application/json" -Body $body
```

### Register equipment

```powershell
$body = @{
  name = "Regulator set"
  category_id = $null
  serial_number = "REG-004"
  brand = "Aqualung"
  regulator_connection = "DIN" # DIN or ETRIER for regulator categories
  size = $null # Use for BCD / wetsuit categories, e.g. "M"
  description = "Primary regulator"
  comment = "New regulator registered"
} | ConvertTo-Json -Compress
Invoke-RestMethod -Uri "http://localhost:8080/api/equipment" -Method Post -ContentType "application/json" -Body $body
```

To assign a category, use the category `id` returned by the previous request instead of `$null`.

### Check out multiple items to one member

Replace the UUID placeholders with real IDs from `/api/members` and `/api/equipment`.

```powershell
$body = @{
  member_id = "MEMBER-UUID"
  equipment_ids = @("EQUIPMENT-UUID-1", "EQUIPMENT-UUID-2")
  due_at = $null
  comment = "Issued for Saturday training"
} | ConvertTo-Json -Compress
Invoke-RestMethod -Uri "http://localhost:8080/api/loans" -Method Post -ContentType "application/json" -Body $body
```

Due date is optional. For a due date, provide an ISO timestamp such as `2026-12-01T18:00:00Z`.

### Return an item

```powershell
$body = @{
  comment = "Returned after pool session; inspection passed"
  condition_status = "ready"
} | ConvertTo-Json -Compress
Invoke-RestMethod -Uri "http://localhost:8080/api/loans/LOAN-UUID/return" -Method Post -ContentType "application/json" -Body $body
```

If damaged, set `condition_status = "needs_repair"` and explain the issue in the comment. Each item in a group is returned separately.

### Mark an item as needing repair / ready

```powershell
$body = @{
  condition_status = "needs_repair"
  comment = "First-stage regulator leaking during inspection"
} | ConvertTo-Json -Compress
Invoke-RestMethod -Uri "http://localhost:8080/api/equipment/EQUIPMENT-UUID/condition" -Method Patch -ContentType "application/json" -Body $body
```

### View an item's event history

```powershell
Invoke-RestMethod -Uri "http://localhost:8080/api/equipment/EQUIPMENT-UUID/history"
```

## Export a complete spreadsheet backup

From the dashboard, click **Export backup (.xlsx)**. This downloads `diving-club-backup.xlsx` with these worksheets:

- `Members`
- `Categories`
- `Equipment`
- `Loans`
- `Equipment History`
- `Backup Info` (format version and export timestamp)

You can also download it directly at `http://localhost:8080/api/export.xlsx`.

The Equipment worksheet also includes `brand`, `regulator_connection` and `size`. The export format version is now 2.

The export is a portable reporting/backup workbook, not a direct PostgreSQL restore. Keep the ID columns intact. A future import feature should validate the workbook, show a preview, detect duplicate IDs and conflicting changes, and require confirmation before applying anything. Until that is implemented, restore a complete system backup using PostgreSQL `pg_dump` rather than editing and re-importing this workbook.

### Back up the database as well (recommended)

The Excel export is convenient for club staff, but it does not preserve every database detail or guarantee a full technical restore. For disaster recovery, also keep a database dump:

```powershell
docker compose exec db pg_dump -U club -d diving_club -F c -f /tmp/diving_club_backup.dump
docker compose cp db:/tmp/diving_club_backup.dump .\\diving_club_backup.dump
```

## API overview

| Method | Path | Purpose |
|---|---|---|
| GET | `/health` | Health check |\n| GET | `/api/export.xlsx` | Download all club data as an Excel workbook |
| GET | `/` | Dashboard |
| GET / POST | `/api/categories` | List / create categories |
| GET / POST | `/api/equipment` | List / register physical items |
| GET | `/api/equipment/:id` | Item details and current borrower |
| GET | `/api/equipment/:id/history` | Item event history |
| PATCH | `/api/equipment/:id/condition` | Change condition; comment required |
| GET / POST | `/api/members` | List / create members |
| PATCH | `/api/members/:id` | Update member diving level |
| GET | `/api/loans` | Loan history |
| POST | `/api/loans` | Checkout one or multiple items |
| POST | `/api/loans/:id/return` | Return one item |

Checkout accepts `equipment_ids` (array); the old single `equipment_id` field is also accepted. Response includes `loan_group_id` and the individual loan records.

## Migration and existing data

Migration `0003_equipment_loans_history.sql` adds the independent condition field, loan grouping and equipment event log. It preserves existing members, equipment and loans. Existing `maintenance` / `unavailable` equipment becomes `needs_repair`; other equipment becomes `ready`. Borrowing status is reconciled against open loans. Past events are not fabricated; each pre-existing item receives a migration note that older history was unavailable.

## Safety and permissions

Checkout is refused unless an item is not already borrowed and its condition is `ready`. Each item can have only one open loan. A return records the comment and condition; it does not silently auto-clear a repair state. History records timestamps and comments and is append-only through these API operations.

**Important:** This starter still has no login system or role-based access control. Comments are required for condition changes, but the API does not yet verify that the caller is an administrator or authorised equipment manager, and does not identify the acting user. Keep it on a trusted local network until authentication, roles, actor audit identity, backups and HTTPS are implemented. Do not expose it publicly.

## Development checks

With Rust installed locally:

```powershell
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```



## Equipment characteristics

When registering equipment, choose a category first:
- **Brand** is available for every item.
- For a category whose name contains **Regulator**, the form displays a connection selector: **DIN** or **Étrier**.
- For categories named **BCD**, **Wetsuit**, **Wet suit**, **Drysuit** or **Dry suit**, the form displays a free-text size field (for example XS, M, L, XL, or a club-specific size).

These fields are optional so existing equipment remains compatible. The database migration adds nullable columns and preserves existing records. The Equipment worksheet in the Excel export includes all three fields.
