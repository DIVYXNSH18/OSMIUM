mod config;
mod database;
mod bot;
mod events;
mod security;
mod verification;
mod moderation;
mod commands;
mod logging;
mod scanner;
mod dashboard;

use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Discord Security Bot...");

    let config = match config::Config::from_env() {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return;
        }
    };

    let db = match database::Database::new(&config.mongodb_uri, &config.mongodb_database).await {
        Ok(d) => Arc::new(d),
        Err(e) => {
            error!("Failed to connect to MongoDB: {}", e);
            return;
        }
    };

    let redis_pool = match deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
    {
        Ok(p) => Arc::new(p),
        Err(e) => {
            error!("Failed to create Redis pool: {}", e);
            return;
        }
    };

    info!("Database connections established");

    let bot_data = bot::BotData::new(config.clone(), db.clone(), redis_pool.clone());
    let bot_handle = tokio::spawn(bot::start_bot(bot_data.clone()));

    let dashboard_handle = tokio::spawn(dashboard::start_dashboard(
        config.clone(),
        db.clone(),
        redis_pool.clone(),
    ));

    info!("Bot and dashboard started successfully");

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = bot_handle => {
            error!("Bot task ended unexpectedly");
        }
        _ = dashboard_handle => {
            error!("Dashboard task ended unexpectedly");
        }
    }

    info!("Shutting down gracefully...");
}
