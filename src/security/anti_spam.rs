use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::collections::VecDeque;
use std::time::Instant;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::logging::logger::Logger;
use tracing::info;

pub async fn check_spam(
    ctx: &Context,
    msg: &Message,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = msg.guild_id.unwrap();
    let key = (guild_id, msg.author.id);
    
    let mut tracker = data.spam_tracker.entry(key).or_insert_with(VecDeque::new);
    let now = Instant::now();
    
    tracker.retain(|t| t.elapsed().as_secs() < config.antispam.time_window_secs);
    tracker.push_back(now);

    let emoji_count = msg.content.chars().filter(|c| {
        (*c as u32 >= 0x1F600 && *c as u32 <= 0x1F64F) ||
        (*c as u32 >= 0x1F300 && *c as u32 <= 0x1F5FF) ||
        (*c as u32 >= 0x1F680 && *c as u32 <= 0x1F6FF) ||
        (*c as u32 >= 0x2600 && *c as u32 <= 0x26FF)
    }).count() as u32;

    if emoji_count > config.antispam.emoji_limit {
        info!("Emoji spam detected from {} in guild {}", msg.author.id, guild_id);
        handle_spam(ctx, msg, config, data.clone()).await?;
        return Ok(());
    }

    if tracker.len() >= config.antispam.message_limit as usize {
        info!("Message spam detected from {} in guild {}", msg.author.id, guild_id);
        handle_spam(ctx, msg, config, data).await?;
    }

    Ok(())
}

async fn handle_spam(
    ctx: &Context,
    msg: &Message,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = msg.delete(&ctx.http).await;

    let guild_id = msg.guild_id.unwrap();
    
    match config.antispam.punishment {
        PunishmentType::Ban => {
            guild_id.ban_with_reason(&ctx.http, msg.author.id, 0, "Anti-Spam: Excessive messages").await?;
        }
        PunishmentType::Kick => {
            guild_id.kick_with_reason(&ctx.http, msg.author.id, "Anti-Spam: Excessive messages").await?;
        }
        PunishmentType::Timeout => {
            if let Ok(mut member) = guild_id.member(&ctx.http, msg.author.id).await {
                member.disable_communication_until_datetime(&ctx.http, serenity::model::Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + 600).unwrap()).await?;
            }
        }
        PunishmentType::Warn => {
            if let Ok(user) = msg.author.id.to_user(&ctx.http).await {
                let _ = user.direct_message(&ctx.http, |m| {
                    m.content("⚠️ You have been warned for spamming.")
                }).await;
            }
        }
        _ => {}
    }

    if let Some(log_channel) = &config.log_channel_id {
        let logger = Logger::new(ctx, log_channel);
        logger.log_spam_detected(&msg.author, msg.channel_id, &config.antispam.punishment).await?;
    }

    Ok(())
}
