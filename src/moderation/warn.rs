use serenity::{client::Context, model::prelude::*};
use crate::database::models::{ModAction, ModerationLog, Warning};
use crate::database::Database;
use chrono::Utc;

pub async fn warn_user(
    ctx: &Context,
    guild_id: GuildId,
    target_id: UserId,
    moderator_id: UserId,
    reason: String,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    database.queries.add_warning(Warning {
        guild_id: guild_id.to_string(),
        user_id: target_id.to_string(),
        moderator_id: moderator_id.to_string(),
        reason: reason.clone(),
        timestamp: Utc::now(),
    }).await?;

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Warn,
        moderator_id: moderator_id.to_string(),
        target_id: target_id.to_string(),
        reason: Some(reason),
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}

pub async fn get_warnings(
    guild_id: GuildId,
    target_id: UserId,
    database: &Database,
) -> Result<Vec<Warning>, Box<dyn std::error::Error + Send + Sync>> {
    Ok(database.queries.get_warnings(&guild_id.to_string(), &target_id.to_string()).await?)
}

pub async fn clear_warnings(
    guild_id: GuildId,
    target_id: UserId,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    database.queries.clear_warnings(&guild_id.to_string(), &target_id.to_string()).await?;
    Ok(())
}
