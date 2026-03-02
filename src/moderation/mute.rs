use serenity::{client::Context, model::prelude::*};
use crate::database::models::{ModAction, ModerationLog};
use crate::database::Database;
use chrono::Utc;

pub async fn mute_user(
    ctx: &Context,
    guild_id: GuildId,
    target_id: UserId,
    moderator_id: UserId,
    duration_seconds: i64,
    reason: Option<String>,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut member = guild_id.member(&ctx.http, target_id).await?;
    
    let until = serenity::model::Timestamp::from_unix_timestamp(Utc::now().timestamp() + duration_seconds)
        .ok_or("Invalid timestamp")?;
    
    member.disable_communication_until_datetime(&ctx.http, until).await?;

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Mute,
        moderator_id: moderator_id.to_string(),
        target_id: target_id.to_string(),
        reason: reason.clone(),
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}

pub async fn unmute_user(
    ctx: &Context,
    guild_id: GuildId,
    target_id: UserId,
    moderator_id: UserId,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut member = guild_id.member(&ctx.http, target_id).await?;
    member.enable_communication(&ctx.http).await?;

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Unmute,
        moderator_id: moderator_id.to_string(),
        target_id: target_id.to_string(),
        reason: None,
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}
