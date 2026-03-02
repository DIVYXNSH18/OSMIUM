use serenity::{client::Context, model::prelude::*};
use crate::database::models::{ModAction, ModerationLog};
use crate::database::Database;
use chrono::Utc;

pub async fn ban_user(
    ctx: &Context,
    guild_id: GuildId,
    target_id: UserId,
    moderator_id: UserId,
    reason: Option<String>,
    delete_days: u8,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    guild_id.ban_with_reason(&ctx.http, target_id, delete_days, reason.as_deref().unwrap_or("No reason provided")).await?;

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Ban,
        moderator_id: moderator_id.to_string(),
        target_id: target_id.to_string(),
        reason: reason.clone(),
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}
