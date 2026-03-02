use std::env;

#[derive(Clone)]
pub struct Config {
    pub discord_token: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub redis_url: String,
    pub dashboard_port: u16,
    pub dashboard_url: String,
    pub jwt_secret: String,
    pub oauth_redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            discord_token: env::var("DISCORD_TOKEN")
                .map_err(|_| "DISCORD_TOKEN not set")?,
            discord_client_id: env::var("DISCORD_CLIENT_ID")
                .map_err(|_| "DISCORD_CLIENT_ID not set")?,
            discord_client_secret: env::var("DISCORD_CLIENT_SECRET")
                .map_err(|_| "DISCORD_CLIENT_SECRET not set")?,
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongodb_database: env::var("MONGODB_DATABASE")
                .unwrap_or_else(|_| "discord_security_bot".to_string()),
            redis_url: env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            dashboard_port: env::var("DASHBOARD_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            dashboard_url: env::var("DASHBOARD_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .map_err(|_| "JWT_SECRET not set")?,
            oauth_redirect_uri: env::var("OAUTH_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string()),
        })
    }
}
