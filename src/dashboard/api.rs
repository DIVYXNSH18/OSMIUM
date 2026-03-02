use axum::{extract::{State, Path}, Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::dashboard::AppState;
use crate::database::models::{GuildConfig, PunishmentType};

#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    pub guild_id: String,
    pub log_channel_id: Option<String>,
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettingsRequest>,
) -> impl IntoResponse {
    let mut config = match state.database.queries.get_guild_config(&payload.guild_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get config").into_response(),
    };
    
    config.log_channel_id = payload.log_channel_id;
    
    match state.database.queries.update_guild_config(&config).await {
        Ok(_) => (StatusCode::OK, "Settings updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update config").into_response(),
    }
}

#[derive(Deserialize)]
pub struct WhitelistRequest {
    pub guild_id: String,
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub action: String,
}

pub async fn update_whitelist(
    State(state): State<AppState>,
    Json(payload): Json<WhitelistRequest>,
) -> impl IntoResponse {
    let mut config = match state.database.queries.get_guild_config(&payload.guild_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get config").into_response(),
    };
    
    if payload.action == "add" {
        if let Some(user_id) = payload.user_id {
            if !config.antinuke.whitelisted_users.contains(&user_id) {
                config.antinuke.whitelisted_users.push(user_id);
            }
        }
        if let Some(role_id) = payload.role_id {
            if !config.antinuke.whitelisted_roles.contains(&role_id) {
                config.antinuke.whitelisted_roles.push(role_id);
            }
        }
    } else if payload.action == "remove" {
        if let Some(user_id) = payload.user_id {
            config.antinuke.whitelisted_users.retain(|id| id != &user_id);
        }
        if let Some(role_id) = payload.role_id {
            config.antinuke.whitelisted_roles.retain(|id| id != &role_id);
        }
    }
    
    match state.database.queries.update_guild_config(&config).await {
        Ok(_) => (StatusCode::OK, "Whitelist updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update whitelist").into_response(),
    }
}

pub async fn get_logs(
    State(state): State<AppState>,
    Path(guild_id): Path<String>,
) -> impl IntoResponse {
    match state.database.queries.get_moderation_logs(&guild_id, 50).await {
        Ok(logs) => (StatusCode::OK, Json(logs)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get logs").into_response(),
    }
}

#[derive(Deserialize)]
pub struct AntiNukeRequest {
    pub guild_id: String,
    pub enabled: bool,
    pub ban_threshold: Option<u32>,
    pub kick_threshold: Option<u32>,
    pub punishment: Option<String>,
}

pub async fn update_antinuke(
    State(state): State<AppState>,
    Json(payload): Json<AntiNukeRequest>,
) -> impl IntoResponse {
    let mut config = match state.database.queries.get_guild_config(&payload.guild_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get config").into_response(),
    };
    
    config.antinuke.enabled = payload.enabled;
    
    if let Some(threshold) = payload.ban_threshold {
        config.antinuke.ban_threshold = threshold;
    }
    
    if let Some(threshold) = payload.kick_threshold {
        config.antinuke.kick_threshold = threshold;
    }
    
    if let Some(punishment) = payload.punishment {
        config.antinuke.punishment = match punishment.as_str() {
            "ban" => PunishmentType::Ban,
            "kick" => PunishmentType::Kick,
            "timeout" => PunishmentType::Timeout,
            _ => PunishmentType::StripRoles,
        };
    }
    
    match state.database.queries.update_guild_config(&config).await {
        Ok(_) => (StatusCode::OK, "Anti-nuke config updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update config").into_response(),
    }
}

#[derive(Deserialize)]
pub struct AntiRaidRequest {
    pub guild_id: String,
    pub enabled: bool,
    pub score_limit: Option<u32>,
    pub lock_channels: Option<bool>,
}

pub async fn update_antiraid(
    State(state): State<AppState>,
    Json(payload): Json<AntiRaidRequest>,
) -> impl IntoResponse {
    let mut config = match state.database.queries.get_guild_config(&payload.guild_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get config").into_response(),
    };
    
    config.antiraid.enabled = payload.enabled;
    
    if let Some(limit) = payload.score_limit {
        config.antiraid.score_limit = limit;
    }
    
    if let Some(lock) = payload.lock_channels {
        config.antiraid.lock_channels = lock;
    }
    
    match state.database.queries.update_guild_config(&config).await {
        Ok(_) => (StatusCode::OK, "Anti-raid config updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update config").into_response(),
    }
}

#[derive(Deserialize)]
pub struct VerificationRequest {
    pub guild_id: String,
    pub enabled: bool,
    pub method: Option<String>,
}

pub async fn update_verification(
    State(state): State<AppState>,
    Json(payload): Json<VerificationRequest>,
) -> impl IntoResponse {
    let mut config = match state.database.queries.get_guild_config(&payload.guild_id).await {
        Ok(c) => c,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get config").into_response(),
    };
    
    config.verification.enabled = payload.enabled;
    
    if let Some(method) = payload.method {
        config.verification.method = if method == "captcha" {
            crate::database::models::VerificationMethod::Captcha
        } else {
            crate::database::models::VerificationMethod::Button
        };
    }
    
    match state.database.queries.update_guild_config(&config).await {
        Ok(_) => (StatusCode::OK, "Verification config updated").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update config").into_response(),
    }
}
