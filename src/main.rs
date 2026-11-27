mod routes;
mod state;
mod error;
mod export;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rustydivingclub=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    let app_state = state::AppState { db: pool };

    let app = Router::new()
        .route("/", get(routes::dashboard))
        .route("/health", get(routes::health))
        .route("/api/export.xlsx", get(|state: axum::extract::State<state::AppState>| async move { export::export_workbook(&state.db).await }))
        .route("/api/equipment", get(routes::list_equipment).post(routes::create_equipment))
        .route("/api/equipment/:id", get(routes::get_equipment))
        .route("/api/equipment/:id/history", get(routes::equipment_history))
        .route("/api/equipment/:id/condition", axum::routing::patch(routes::change_condition))
        .route("/api/categories", get(routes::list_categories).post(routes::create_category))
        .route("/api/members", get(routes::list_members).post(routes::create_member))
        .route("/api/members/:id", axum::routing::patch(routes::update_member_level))
        .route("/api/loans", get(routes::list_loans).post(routes::checkout))
        .route("/api/loans/:id/return", axum::routing::post(routes::return_equipment))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "8080".into()).parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Diving Club Manager listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}
