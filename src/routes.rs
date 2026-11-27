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
    pub diving_level: Option<String>,
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
    pub diving_level: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberLevel {
    pub diving_level: String,
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
    let equipment: i64 = sqlx::query_scalar("SELECT count(*) FROM equipment").fetch_one(&s.db).await?;
    let available: i64 = sqlx::query_scalar("SELECT count(*) FROM equipment WHERE status = 'available'").fetch_one(&s.db).await?;
    let members: i64 = sqlx::query_scalar("SELECT count(*) FROM members WHERE active = true").fetch_one(&s.db).await?;
    let loans: i64 = sqlx::query_scalar("SELECT count(*) FROM loans WHERE returned_at IS NULL").fetch_one(&s.db).await?;
    let html = r##"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Diving Club Manager</title>
<style>
:root{--bg:#071923;--panel:#102936;--line:#254655;--text:#edf7fa;--muted:#91abb7;--mint:#64e0c0}*{box-sizing:border-box}body{margin:0;background:radial-gradient(ellipse at 85% -10%,#174052 0,transparent 38%),var(--bg);color:var(--text);font:15px/1.5 Inter,system-ui,sans-serif}.shell{max-width:1220px;margin:auto;padding:34px 26px 60px}.top{display:flex;justify-content:space-between;align-items:center;gap:18px;margin-bottom:32px}.brand{display:flex;align-items:center;gap:13px}.logo{width:46px;height:46px;border-radius:15px;background:linear-gradient(145deg,#68e2c2,#3c9ec0);display:grid;place-items:center;color:#06222b;font-size:23px;font-weight:900}.eyebrow{font-size:11px;letter-spacing:.16em;color:var(--mint);font-weight:800}h1{font-size:clamp(25px,4vw,36px);line-height:1.1;margin:4px 0}.sub,.muted{color:var(--muted)}.sub{margin:0}.top-link{color:var(--mint);text-decoration:none;border:1px solid var(--line);padding:10px 14px;border-radius:11px}.stats{display:grid;grid-template-columns:repeat(4,minmax(0,1fr));gap:14px;margin-bottom:26px}.stat,.panel{background:linear-gradient(145deg,#153443f5,#0d232ff5);border:1px solid var(--line);border-radius:18px;box-shadow:0 12px 35px #0002}.stat{padding:19px 20px}.stat-label{color:var(--muted);font-size:13px}.stat strong{display:block;font-size:32px;letter-spacing:-.04em;margin-top:5px}.stat:nth-child(2) strong{color:var(--mint)}.content{display:grid;grid-template-columns:minmax(0,1.6fr) minmax(280px,.8fr);gap:18px;align-items:start}.panel{padding:22px}.panel-head{display:flex;justify-content:space-between;align-items:center;gap:12px;margin-bottom:18px}h2{font-size:18px;margin:0}.tools{display:flex;gap:9px;flex-wrap:wrap}input,select{background:#091f2a;border:1px solid var(--line);border-radius:10px;color:var(--text);padding:11px 12px;font:inherit;min-width:0}input:focus,select:focus{outline:2px solid #64e0c066;border-color:var(--mint)}.search{width:min(230px,100%)}.table-wrap{overflow-x:auto}table{width:100%;border-collapse:collapse;white-space:nowrap}th{text-align:left;color:var(--muted);font-size:11px;text-transform:uppercase;letter-spacing:.1em;padding:11px 10px;border-bottom:1px solid var(--line)}td{padding:14px 10px;border-bottom:1px solid #24425088}tbody tr:last-child td{border-bottom:0}.member-name{font-weight:700}.member-email{font-size:12px;color:var(--muted)}.level-select{background:#123e3d;border:1px solid #28675d;border-radius:9px;color:#8ef0d3;font-weight:800;font-size:12px;padding:6px 8px}.empty{padding:28px;text-align:center;color:var(--muted)}.form-grid{display:grid;gap:12px}label{display:grid;gap:6px;color:#c4d7de;font-size:13px}button{border:0;border-radius:10px;background:var(--mint);color:#06232b;font-weight:800;padding:12px 15px;font:inherit;cursor:pointer}button:hover{filter:brightness(1.06)}.notice{min-height:20px;color:var(--mint);font-size:13px}.level-guide{margin-top:18px;padding-top:16px;border-top:1px solid var(--line)}.level-guide p{font-size:12px;color:var(--muted);margin:6px 0 10px}.chips{display:flex;gap:6px;flex-wrap:wrap}.chip{font-size:11px;color:#bdd2da;background:#1a3a49;border-radius:7px;padding:4px 7px}@media(max-width:850px){.content{grid-template-columns:1fr}}@media(max-width:600px){.shell{padding:22px 15px}.stats{grid-template-columns:repeat(2,minmax(0,1fr))}.top{align-items:flex-start}.top-link{display:none}.panel{padding:16px}.panel-head{align-items:flex-start;flex-direction:column}.tools{width:100%}.search{width:100%}}
</style></head><body><main class="shell"><header class="top"><div class="brand"><div class="logo">≈</div><div><div class="eyebrow">DIVING CLUB / OPERATIONS</div><h1>Diving Club Manager</h1><p class="sub">Your club, at a glance. Ready for the next dive.</p></div></div><a class="top-link" href="/api/loans">View loan API ↗</a></header>
<section class="stats"><article class="stat"><div class="stat-label">Registered members</div><strong>{{MEMBERS}}</strong></article><article class="stat"><div class="stat-label">Equipment available</div><strong>{{AVAILABLE}}<span style="font-size:14px;color:var(--muted)"> / {{EQUIPMENT}}</span></strong></article><article class="stat"><div class="stat-label">Currently on loan</div><strong>{{LOANS}}</strong></article><article class="stat"><div class="stat-label">Equipment inventory</div><strong>{{EQUIPMENT}}</strong></article></section>
<section class="content"><article class="panel"><div class="panel-head"><div><h2>Member directory</h2><div class="muted" style="font-size:13px">Manage members and their diving qualifications</div></div><div class="tools"><input id="search" class="search" type="search" placeholder="Search members…" aria-label="Search members"><select id="level-filter" aria-label="Filter by diving level"><option value="">All levels</option><option>N1</option><option>PN1</option><option>N2</option><option>PN2</option><option>N3</option><option>PN3</option><option>N4</option><option>PN4</option><option>E1</option><option>E2</option><option>E3</option><option>E4</option><option value="unassigned">Unassigned</option></select></div></div><div class="table-wrap"><table><thead><tr><th>Member</th><th>Diving level</th><th>Status</th></tr></thead><tbody id="members"><tr><td colspan="3" class="empty">Loading members…</td></tr></tbody></table></div><div id="member-count" class="muted" style="font-size:12px;margin-top:12px"></div></article>
<aside class="panel"><div class="panel-head"><div><h2>Add a member</h2><div class="muted" style="font-size:13px">Create a club member profile</div></div></div><form id="member-form" class="form-grid"><label>Full name<input name="full_name" required maxlength="160" placeholder="e.g. Alex Martin"></label><label>Email address<input name="email" type="email" placeholder="alex@example.com"></label><label>Phone<input name="phone" type="tel" placeholder="Optional"></label><label>Diving level<select name="diving_level" required><option value="" disabled selected>Choose a level…</option><option>N1</option><option>PN1</option><option>N2</option><option>PN2</option><option>N3</option><option>PN3</option><option>N4</option><option>PN4</option><option>E1</option><option>E2</option><option>E3</option><option>E4</option></select></label><button type="submit">Add member</button><div id="form-notice" class="notice" role="status"></div></form><div class="level-guide"><strong style="font-size:13px">Available qualifications</strong><p>Choose the member's current qualification from the approved list.</p><div class="chips">{{LEVEL_CHIPS}}</div></div></aside></section></main>
<script>
const tbody=document.getElementById('members'),search=document.getElementById('search'),filter=document.getElementById('level-filter');let allMembers=[];
function renderMembers(){const q=search.value.trim().toLocaleLowerCase(),level=filter.value;const rows=allMembers.filter(m=>(m.full_name+' '+(m.email||'')).toLocaleLowerCase().includes(q)&&(level===''||(level==='unassigned'?!m.diving_level:m.diving_level===level)));tbody.replaceChildren();if(!rows.length){const tr=document.createElement('tr'),td=document.createElement('td');td.colSpan=3;td.className='empty';td.textContent=allMembers.length?'No members match these filters.':'No members yet. Add your first member using the form.';tr.append(td);tbody.append(tr)}for(const m of rows){const tr=document.createElement('tr'),name=document.createElement('td'),nm=document.createElement('div'),email=document.createElement('div'),lev=document.createElement('td'),status=document.createElement('td'),badge=document.createElement('select');nm.className='member-name';nm.textContent=m.full_name;email.className='member-email';email.textContent=m.email||'No email provided';name.append(nm,email);badge.className='level-select';badge.setAttribute('aria-label','Diving level for '+m.full_name);for(const level of ['N1','PN1','N2','PN2','N3','PN3','N4','PN4','E1','E2','E3','E4']){const opt=document.createElement('option');opt.value=level;opt.textContent=level;badge.append(opt)}if(m.diving_level){badge.value=m.diving_level}else{const opt=document.createElement('option');opt.value='';opt.textContent='Assign level…';opt.disabled=true;opt.selected=true;badge.prepend(opt)}badge.dataset.memberId=m.id;lev.append(badge);status.textContent=m.active?'Active':'Inactive';status.style.color=m.active?'var(--mint)':'var(--muted)';tr.append(name,lev,status);tbody.append(tr)}document.getElementById('member-count').textContent=rows.length+' of '+allMembers.length+' members'}
async function loadMembers(){try{const r=await fetch('/api/members');if(!r.ok)throw new Error(await r.text());allMembers=await r.json();renderMembers()}catch(e){tbody.innerHTML='<tr><td colspan="3" class="empty">Could not load members. Check the server and refresh.</td></tr>';console.error(e)}}
search.addEventListener('input',renderMembers);filter.addEventListener('change',renderMembers);tbody.addEventListener('change',async e=>{if(!e.target.matches('.level-select'))return;const select=e.target;try{const r=await fetch('/api/members/'+select.dataset.memberId,{method:'PATCH',headers:{'Content-Type':'application/json'},body:JSON.stringify({diving_level:select.value})});if(!r.ok)throw new Error(await r.text());const updated=await r.json();const member=allMembers.find(m=>m.id===updated.id);if(member)member.diving_level=updated.diving_level;document.getElementById('form-notice').textContent='Diving level updated.'}catch(err){document.getElementById('form-notice').textContent='Could not update level: '+err.message;loadMembers()}});
document.getElementById('member-form').addEventListener('submit',async e=>{e.preventDefault();const f=new FormData(e.currentTarget),payload=Object.fromEntries(f.entries()),notice=document.getElementById('form-notice');for(const k of ['email','phone'])if(!payload[k])payload[k]=null;try{const r=await fetch('/api/members',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(payload)});if(!r.ok)throw new Error(await r.text());const member=await r.json();allMembers.unshift(member);renderMembers();e.currentTarget.reset();notice.textContent='Member added successfully.'}catch(err){notice.textContent='Could not add member: '+err.message}});loadMembers();
</script></body></html>"##;
    let levels = ["N1", "PN1", "N2", "PN2", "N3", "PN3", "N4", "PN4", "E1", "E2", "E3", "E4"];
    let chips = levels.iter().map(|l| format!("<span class=\"chip\">{l}</span>")).collect::<String>();
    let html = html.replace("{{MEMBERS}}", &members.to_string()).replace("{{AVAILABLE}}", &available.to_string()).replace("{{EQUIPMENT}}", &equipment.to_string()).replace("{{LOANS}}", &loans.to_string()).replace("{{LEVEL_CHIPS}}", &chips);
    Ok(Html(html))
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
        "SELECT id, full_name, email, phone, diving_level, active, created_at FROM members ORDER BY full_name"
    ).fetch_all(&s.db).await?;
    Ok(Json(rows))
}

pub async fn create_member(
    State(s): State<AppState>, Json(input): Json<NewMember>
) -> Result<(StatusCode, Json<Member>), AppError> {
    input.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    const LEVELS: [&str; 12] = ["N1", "PN1", "N2", "PN2", "N3", "PN3", "N4", "PN4", "E1", "E2", "E3", "E4"];
    if !LEVELS.contains(&input.diving_level.as_str()) {
        return Err(AppError::BadRequest("invalid diving_level; choose one of N1, PN1, N2, PN2, N3, PN3, N4, PN4, E1, E2, E3, or E4".into()));
    }
    let row = sqlx::query_as::<_, Member>(
        "INSERT INTO members (full_name, email, phone, diving_level) VALUES ($1,$2,$3,$4) RETURNING id, full_name, email, phone, diving_level, active, created_at"
    ).bind(input.full_name.trim()).bind(input.email).bind(input.phone).bind(input.diving_level)
     .fetch_one(&s.db).await?;
    Ok((StatusCode::CREATED, Json(row)))
}

pub async fn update_member_level(
    State(s): State<AppState>, Path(id): Path<Uuid>, Json(input): Json<UpdateMemberLevel>
) -> Result<Json<Member>, AppError> {
    const LEVELS: [&str; 12] = ["N1", "PN1", "N2", "PN2", "N3", "PN3", "N4", "PN4", "E1", "E2", "E3", "E4"];
    if !LEVELS.contains(&input.diving_level.as_str()) {
        return Err(AppError::BadRequest("invalid diving_level".into()));
    }
    let row = sqlx::query_as::<_, Member>(
        "UPDATE members SET diving_level = $1 WHERE id = $2 RETURNING id, full_name, email, phone, diving_level, active, created_at"
    ).bind(input.diving_level).bind(id).fetch_optional(&s.db).await?.ok_or(AppError::NotFound)?;
    Ok(Json(row))
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
