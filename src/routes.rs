use axum::{extract::{Path, State}, http::StatusCode, response::Html, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::{error::AppError, state::AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Equipment {
    pub id: Uuid,
    pub qr_uuid: Uuid,
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub serial_number: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Member {
    pub id: Uuid,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Loan {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub equipment_name: String,
    pub member_id: Uuid,
    pub member_name: String,
    pub checked_out_at: DateTime<Utc>,
    pub due_at: Option<DateTime<Utc>>,
    pub returned_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewEquipment {
    #[validate(length(min = 1, max = 160))]
    pub name: String,
    pub category_id: Option<Uuid>,
    pub description: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewMember {
    #[validate(length(min = 1, max = 160))]
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub equipment_id: Uuid,
    pub member_id: Uuid,
    pub due_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

pub async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

pub async fn dashboard(State(s): State<AppState>) -> Result<Html<String>, AppError> {
    let equipment: i64 = sqlx::query_scalar("SELECT count(*) FROM equipment")
        .fetch_one(&s.db).await?;
    let available: i64 = sqlx::query_scalar("SELECT count(*) FROM equipment WHERE status = 'available'")
        .fetch_one(&s.db).await?;
    let members: i64 = sqlx::query_scalar("SELECT count(*) FROM members WHERE active = true")
        .fetch_one(&s.db).await?;
    let loans: i64 = sqlx::query_scalar("SELECT count(*) FROM loans WHERE returned_at IS NULL")
        .fetch_one(&s.db).await?;
    Ok(Html(format!(r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width">
<title>Diving Club Manager</title>
<style>
:root{{font-family:system-ui,sans-serif;color:#e8f0f5;background:#0b1720}}
body{{max-width:1000px;margin:48px auto;padding:0 20px}}h1{{font-size:2rem}}
small{{color:#9eb2c1}}.grid{{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:16px;margin:28px 0}}
article{{background:#142633;border:1px solid #294252;border-radius:14px;padding:22px}}
strong{{display:block;font-size:2.2rem;color:#5dd6c0}}a{{color:#5dd6c0;margin-right:18px}}
</style></head><body><small>DIVING CLUB / OPERATIONS</small><h1>Diving Club Manager</h1>
<p>Equipment, members and equipment loans in one place.</p>
<div class="grid">
<article><small>Total equipment</small><strong>{equipment}</strong></article>
<article><small>Available equipment</small><strong>{available}</strong></article>
<article><small>Active members</small><strong>{members}</strong></article>
<article><small>Open loans</small><strong>{loans}</strong></article>
</div><nav><a href="/api/equipment">Equipment API</a><a href="/api/members">Members API</a><a href="/api/loans">Loan history API</a></nav>
</body></html>"#)))
}

pub async fn list_equipment(State(s): State<AppState>) -> Result<Json<Vec<Equipment>>, AppError> {
    let rows = sqlx::query_as::<_, Equipment>(
        "SELECT id, qr_uuid, category_id, name, description, serial_number, status, created_at FROM equipment ORDER BY name"
    ).fetch_all(&s.db).await?;
    Ok(Json(rows))
}

pub async fn get_equipment(State(s): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Equipment>, AppError> {
    let row = sqlx::query_as::<_, Equipment>(
        "SELECT id, qr_uuid, category_id, name, description, serial_number, status, created_at FROM equipment WHERE id = $1"
    ).bind(id).fetch_optional(&s.db).await?.ok_or(AppError::NotFound)?;
    Ok(Json(row))
}

pub async fn create_equipment(
    State(s): State<AppState>, Json(input): Json<NewEquipment>
) -> Result<(StatusCode, Json<Equipment>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let row = sqlx::query_as::<_, Equipment>(
        "INSERT INTO equipment (name, category_id, description, serial_number) VALUES ($1,$2,$3,$4) RETURNING id, qr_uuid, category_id, name, description, serial_number, status, created_at"
    ).bind(input.name.trim()).bind(input.category_id).bind(input.description)
     .bind(input.serial_number).fetch_one(&s.db).await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn list_members(State(s): State<AppState>) -> Result<Json<Vec<Member>>, AppError> {
    let rows = sqlx::query_as::<_, Member>(
        "SELECT id, full_name, email, phone, active, created_at FROM members ORDER BY full_name"
    ).fetch_all(&s.db).await?;
    Ok(Json(rows))
}

pub async fn create_member(
    State(s): State<AppState>, Json(input): Json<NewMember>
) -> Result<(StatusCode, Json<Member>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    let row = sqlx::query_as::<_, Member>(
        "INSERT INTO members (full_name, email, phone) VALUES ($1,$2,$3) RETURNING id, full_name, email, phone, active, created_at"
    ).bind(input.full_name.trim()).bind(input.email).bind(input.phone)
     .fetch_one(&s.db).await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn list_loans(State(s): State<AppState>) -> Result<Json<Vec<Loan>>, AppError> {
    let rows = sqlx::query_as::<_, Loan>(r#"
        SELECT l.id, l.equipment_id, e.name AS equipment_name, l.member_id,
               m.full_name AS member_name, l.checked_out_at, l.due_at,
               l.returned_at, l.notes
        FROM loans l JOIN equipment e ON e.id=l.equipment_id
        JOIN members m ON m.id=l.member_id
        ORDER BY l.checked_out_at DESC
    "#).fetch_all(&s.db).await?;
    Ok(Json(rows))
}

pub async fn checkout(
    State(s): State<AppState>, Json(input): Json<CheckoutRequest>
) -> Result<(StatusCode, Json<Loan>), AppError> {
    let mut tx = s.db.begin().await?;
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM equipment WHERE id=$1 FOR UPDATE"
    ).bind(input.equipment_id).fetch_optional(&mut *tx).await?;
    match status.as_deref() {
        None => return Err(AppError::NotFound),
        Some("available") => {},
        Some(other) => return Err(AppError::Conflict(format!("equipment is {other}"))),
    }
    let member_active: Option<bool> = sqlx::query_scalar(
        "SELECT active FROM members WHERE id=$1"
    ).bind(input.member_id).fetch_optional(&mut *tx).await?;
    if member_active != Some(true) {
        return Err(AppError::BadRequest("member does not exist or is inactive".into()));
    }
    let loan_id = Uuid::new_v4();
    sqlx::query("INSERT INTO loans (id,equipment_id,member_id,due_at,notes) VALUES ($1,$2,$3,$4,$5)")
        .bind(loan_id).bind(input.equipment_id).bind(input.member_id)
        .bind(input.due_at).bind(input.notes).execute(&mut *tx).await?;
    sqlx::query("UPDATE equipment SET status='checked_out', updated_at=now() WHERE id=$1")
        .bind(input.equipment_id).execute(&mut *tx).await?;
    tx.commit().await?;
    let row = sqlx::query_as::<_, Loan>(r#"
        SELECT l.id, l.equipment_id, e.name AS equipment_name, l.member_id,
               m.full_name AS member_name, l.checked_out_at, l.due_at,
               l.returned_at, l.notes FROM loans l
        JOIN equipment e ON e.id=l.equipment_id JOIN members m ON m.id=l.member_id
        WHERE l.id=$1
    "#).bind(loan_id).fetch_one(&s.db).await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn return_equipment(
    State(s): State<AppState>, Path(id): Path<Uuid>
) -> Result<Json<Loan>, AppError> {
    let mut tx = s.db.begin().await?;
    let equipment_id: Option<Uuid> = sqlx::query_scalar(
        "UPDATE loans SET returned_at=now() WHERE id=$1 AND returned_at IS NULL RETURNING equipment_id"
    ).bind(id).fetch_optional(&mut *tx).await?;
    let equipment_id = equipment_id.ok_or(AppError::NotFound)?;
    sqlx::query("UPDATE equipment SET status='available', updated_at=now() WHERE id=$1 AND status='checked_out'")
        .bind(equipment_id).execute(&mut *tx).await?;
    tx.commit().await?;
    let row = sqlx::query_as::<_, Loan>(r#"
        SELECT l.id, l.equipment_id, e.name AS equipment_name, l.member_id,
               m.full_name AS member_name, l.checked_out_at, l.due_at,
               l.returned_at, l.notes FROM loans l
        JOIN equipment e ON e.id=l.equipment_id JOIN members m ON m.id=l.member_id
        WHERE l.id=$1
    "#).bind(id).fetch_one(&s.db).await?;
    Ok(Json(row))
}

#[cfg(test)]
mod tests {
    #[test]
    fn qr_payload_has_stable_prefix_and_uuid() {
        let id = uuid::Uuid::nil();
        assert_eq!(format!("DCM:{id}"), "DCM:00000000-0000-0000-0000-000000000000");
    }
}
