pub mod routes;
pub mod auth;
pub mod api;

use std::sync::Arc;
use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
use crate::config::Config;
use crate::database::Database;

pub async fn start_dashboard(
    config: Arc<Config>,
    database: Arc<Database>,
    redis_pool: Arc<deadpool_redis::Pool>,
) {
    let app = Router::new()
        .route("/", get(routes::landing))
        .route("/dashboard", get(routes::dashboard))
        .route("/dashboard/:guild_id", get(routes::guild_settings))
        .route("/api/settings", post(api::update_settings))
        .route("/api/whitelist", post(api::update_whitelist))
        .route("/api/logs/:guild_id", get(api::get_logs))
        .route("/api/antinuke", post(api::update_antinuke))
        .route("/api/antiraid", post(api::update_antiraid))
        .route("/api/verification", post(api::update_verification))
        .route("/auth/login", get(auth::login))
        .route("/auth/callback", get(auth::callback))
        .layer(CorsLayer::permissive())
        .with_state(AppState {
            config,
            database,
            redis_pool,
        });

    let addr = format!("0.0.0.0:{}", config.dashboard_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    tracing::info!("Dashboard listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub redis_pool: Arc<deadpool_redis::Pool>,
}
