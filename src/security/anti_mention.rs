use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::whitelist;
use crate::logging::logger::Logger;
use tracing::info;

pub async fn check_mentions(
    ctx: &Context,
    msg: &Message,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = msg.guild_id.unwrap();
    
    if msg.mention_everyone {
        if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
            if !whitelist::is_mention_whitelisted(&member, config) {
                info!("Unauthorized @everyone/@here from {} in guild {}", msg.author.id, guild_id);
                handle_mention_abuse(ctx, msg, config, data).await?;
                return Ok(());
            }
        }
    }

    if msg.mentions.len() >= config.antimention.mention_limit as usize {
        if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
            if !whitelist::is_mention_whitelisted(&member, config) {
                info!("Mass mention from {} in guild {}: {} mentions", msg.author.id, guild_id, msg.mentions.len());
                handle_mention_abuse(ctx, msg, config, data).await?;
            }
        }
    }

    Ok(())
}

async fn handle_mention_abuse(
    ctx: &Context,
    msg: &Message,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _ = msg.delete(&ctx.http).await;

    let guild_id = msg.guild_id.unwrap();
    
    match config.antimention.punishment {
        PunishmentType::Ban => {
            guild_id.ban_with_reason(&ctx.http, msg.author.id, 0, "Anti-Mention: Mass mention abuse").await?;
        }
        PunishmentType::Kick => {
            guild_id.kick_with_reason(&ctx.http, msg.author.id, "Anti-Mention: Mass mention abuse").await?;
        }
        PunishmentType::Timeout => {
            if let Ok(mut member) = guild_id.member(&ctx.http, msg.author.id).await {
                member.disable_communication_until_datetime(&ctx.http, serenity::model::Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + 3600).unwrap()).await?;
            }
        }
        PunishmentType::Warn => {
            if let Ok(user) = msg.author.id.to_user(&ctx.http).await {
                let _ = user.direct_message(&ctx.http, |m| {
                    m.content("⚠️ You have been warned for mass mention abuse.")
                }).await;
            }
        }
        _ => {}
    }

    if let Some(log_channel) = &config.log_channel_id {
        let logger = Logger::new(ctx, log_channel);
        logger.log_mention_abuse(&msg.author, msg.channel_id, msg.mentions.len(), &config.antimention.punishment).await?;
    }

    Ok(())
}
