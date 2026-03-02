use serenity::{client::Context, model::prelude::*};
use crate::database::models::{ModAction, ModerationLog};
use crate::database::Database;
use chrono::Utc;

pub async fn clear_messages(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
    moderator_id: UserId,
    amount: u64,
    target_user: Option<UserId>,
    database: &Database,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let messages = channel_id.messages(&ctx.http, |m| m.limit(amount)).await?;
    
    let to_delete: Vec<MessageId> = if let Some(user_id) = target_user {
        messages.iter()
            .filter(|m| m.author.id == user_id)
            .map(|m| m.id)
            .collect()
    } else {
        messages.iter().map(|m| m.id).collect()
    };

    let count = to_delete.len() as u64;
    
    if !to_delete.is_empty() {
        channel_id.delete_messages(&ctx.http, &to_delete).await?;
    }

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Clear,
        moderator_id: moderator_id.to_string(),
        target_id: target_user.map(|u| u.to_string()).unwrap_or_else(|| "all".to_string()),
        reason: Some(format!("Cleared {} messages", count)),
        timestamp: Utc::now(),
    }).await?;

    Ok(count)
}
