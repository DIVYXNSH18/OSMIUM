pub mod ban;
pub mod kick;
pub mod mute;
pub mod clear;
pub mod lock;
pub mod warn;

use serenity::{client::Context, model::prelude::*};
use crate::database::models::GuildConfig;

pub async fn check_permissions(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let member = guild_id.member(&ctx.http, user_id).await?;
    let permissions = member.permissions(&ctx.cache)?;
    
    Ok(permissions.administrator() || permissions.moderate_members() || permissions.ban_members())
}

pub async fn dm_user(
    ctx: &Context,
    user_id: UserId,
    action: &str,
    reason: Option<&str>,
    guild_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::{CreateMessage, CreateEmbed};
    
    if let Ok(user) = user_id.to_user(&ctx.http).await {
        let reason_text = reason.unwrap_or("No reason provided");
        let embed = CreateEmbed::new()
            .title(format!("⚠️ Moderation Action: {}", action))
            .description(format!("**Server:** {}\n**Reason:** {}", guild_name, reason_text))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        let _ = user.direct_message(&ctx.http, message).await;
    }
    Ok(())
}
