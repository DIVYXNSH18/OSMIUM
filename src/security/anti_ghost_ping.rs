use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::logging::logger::Logger;
use tracing::info;

pub async fn check_ghost_ping(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    guild_id: GuildId,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some((_, cached)) = data.message_cache.remove(&deleted_message_id) {
        if !cached.mentions.is_empty() {
            info!("Ghost ping detected from {} in guild {}", cached.author_id, guild_id);

            let mentions_str = cached.mentions.iter()
                .map(|id| format!("<@{}>", id))
                .collect::<Vec<_>>()
                .join(", ");

            let _ = channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("👻 Ghost Ping Detected")
                        .description(format!(
                            "**Author:** <@{}>\n**Mentioned:** {}\n**Content:** {}\n**Time:** <t:{}:R>",
                            cached.author_id,
                            mentions_str,
                            if cached.content.is_empty() { "*No text content*" } else { &cached.content },
                            chrono::Utc::now().timestamp()
                        ))
                        .color(0xFFA500)
                        .timestamp(chrono::Utc::now())
                })
            }).await;

            match config.antighost.punishment {
                PunishmentType::Timeout => {
                    if let Ok(mut member) = guild_id.member(&ctx.http, cached.author_id).await {
                        let _ = member.disable_communication_until_datetime(&ctx.http, serenity::model::Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + 600).unwrap()).await;
                    }
                }
                PunishmentType::Warn => {
                    if let Ok(user) = cached.author_id.to_user(&ctx.http).await {
                        let _ = user.direct_message(&ctx.http, |m| {
                            m.content("⚠️ You have been warned for ghost pinging.")
                        }).await;
                    }
                }
                _ => {}
            }

            if let Some(log_channel) = &config.log_channel_id {
                let logger = Logger::new(ctx, log_channel);
                logger.log_ghost_ping(cached.author_id, channel_id, &cached.mentions).await?;
            }
        }
    }

    Ok(())
}
