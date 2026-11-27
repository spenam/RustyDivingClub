use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::Response,
};
use rust_xlsxwriter::{Workbook, Worksheet};
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::error::AppError;

async fn json_rows(db: &PgPool, sql: &str) -> Result<Vec<Vec<String>>, AppError> {
    let rows = sqlx::query(sql).fetch_all(db).await?;
    let mut output = Vec::with_capacity(rows.len());
    for row in rows {
        let value: Value = row.try_get("data")?;
        let cells = value.as_array().ok_or_else(|| {
            AppError::BadRequest("Unexpected export row format".into())
        })?;
        output.push(cells.iter().map(|cell| match cell {
            Value::Null => String::new(),
            Value::String(s) => s.clone(),
            other => other.to_string(),
        }).collect());
    }
    Ok(output)
}

fn add_sheet(
    workbook: &mut Workbook,
    name: &str,
    headers: &[&str],
    rows: Vec<Vec<String>>,
) -> Result<(), AppError> {
    let worksheet: &mut Worksheet = workbook.add_worksheet();
    worksheet.set_name(name)
        .map_err(|e| AppError::Export(e.to_string()))?;
    for (col, header_text) in headers.iter().enumerate() {
        worksheet.write_string(0, col as u16, *header_text)
            .map_err(|e| AppError::Export(e.to_string()))?;
    }
    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, value) in row.iter().enumerate() {
            worksheet.write_string((row_index + 1) as u32, col_index as u16, value)
                .map_err(|e| AppError::Export(e.to_string()))?;
        }
    }
    worksheet.autofit();
    Ok(())
}

pub async fn export_workbook(db: &PgPool) -> Result<Response, AppError> {
    let mut workbook = Workbook::new();

    let members = json_rows(db, r#"
        SELECT jsonb_build_array(id::text, full_name, email, phone, diving_level,
                                 active::text, created_at::text) AS data
        FROM members ORDER BY full_name, id
    "#).await?;
    add_sheet(&mut workbook, "Members",
        &["member_id","full_name","email","phone","diving_level","active","created_at"],
        members)?;

    let categories = json_rows(db, r#"
        SELECT jsonb_build_array(id::text, name, description) AS data
        FROM equipment_categories ORDER BY name, id
    "#).await?;
    add_sheet(&mut workbook, "Categories",
        &["category_id","name","description"], categories)?;

    let equipment = json_rows(db, r#"
        SELECT jsonb_build_array(e.id::text, e.qr_uuid::text, e.category_id::text,
            c.name, e.name, e.brand, e.regulator_connection, e.size,
            e.description, e.serial_number, e.status, e.condition_status,
            e.created_at::text, e.updated_at::text, l.member_id::text,
            m.full_name, l.checked_out_at::text, l.due_at::text) AS data
        FROM equipment e
        LEFT JOIN equipment_categories c ON c.id=e.category_id
        LEFT JOIN loans l ON l.equipment_id=e.id AND l.returned_at IS NULL
        LEFT JOIN members m ON m.id=l.member_id
        ORDER BY e.name, e.id
    "#).await?;
    add_sheet(&mut workbook, "Equipment",
        &["equipment_id","qr_uuid","category_id","category_name","name","brand",
          "regulator_connection","size","description","serial_number",
          "borrowing_status","condition_status","created_at","updated_at",
          "current_member_id","current_member_name","current_checked_out_at",
          "current_due_at"], equipment)?;

    let loans = json_rows(db, r#"
        SELECT jsonb_build_array(l.id::text, l.loan_group_id::text,
            l.equipment_id::text, e.name, l.member_id::text, m.full_name,
            l.checked_out_at::text, l.due_at::text, l.returned_at::text,
            l.notes) AS data
        FROM loans l JOIN equipment e ON e.id=l.equipment_id
        JOIN members m ON m.id=l.member_id
        ORDER BY l.checked_out_at, l.id
    "#).await?;
    add_sheet(&mut workbook, "Loans",
        &["loan_id","loan_group_id","equipment_id","equipment_name",
          "member_id","member_name","checked_out_at","due_at","returned_at","notes"],
        loans)?;

    let history = json_rows(db, r#"
        SELECT jsonb_build_array(id::text, equipment_id::text, loan_id::text,
            event_type, from_state::text, to_state::text, comment,
            occurred_at::text) AS data
        FROM equipment_events ORDER BY occurred_at, id
    "#).await?;
    add_sheet(&mut workbook, "Equipment History",
        &["event_id","equipment_id","loan_id","event_type","from_state_json",
          "to_state_json","comment","occurred_at"], history)?;

    let exported_at = chrono::Utc::now().to_rfc3339();
    add_sheet(&mut workbook, "Backup Info",
        &["field","value"],
        vec![
            vec!["format".into(), "Diving Club Manager workbook backup".into()],
            vec!["format_version".into(), "2".into()],
            vec!["exported_at_utc".into(), exported_at],
            vec!["record_ids".into(), "UUIDs are stable identifiers; keep them unchanged for future import".into()],
            vec!["import_policy".into(), "Future imports should be validated and previewed before applying; this workbook is not a direct database restore file".into()],
        ])?;

    let bytes = workbook.save_to_buffer()
        .map_err(|e| AppError::Export(e.to_string()))?;
    let mut response = Response::new(Body::from(bytes));
    *response.status_mut() = StatusCode::OK;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    );
    response.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("attachment; filename=\"diving-club-backup.xlsx\""),
    );
    Ok(response)
}
