use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::collections::HashMap;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::whitelist;
use crate::logging::logger::Logger;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

pub async fn check_emoji_changes(
    ctx: &Context,
    guild_id: GuildId,
    current_state: HashMap<EmojiId, Emoji>,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::EmojiDelete), None, None, Some(5)).await {
        for entry in audit_logs.entries.values() {
            if let Some(executor_id) = entry.user_id {
                let member = guild_id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    continue;
                }

                info!("Unauthorized emoji deletion by {} in guild {}", executor_id, guild_id);

                apply_emoji_punishment(ctx, guild_id, executor_id, &config.antiemoji.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_emoji_delete(executor_id, &config.antiemoji.punishment).await?;
                }
            }
        }
    }

    if let Ok(audit_logs) = guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::EmojiUpdate), None, None, Some(5)).await {
        for entry in audit_logs.entries.values() {
            if let Some(executor_id) = entry.user_id {
                let member = guild_id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    continue;
                }

                info!("Unauthorized emoji rename by {} in guild {}", executor_id, guild_id);

                apply_emoji_punishment(ctx, guild_id, executor_id, &config.antiemoji.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_emoji_rename(executor_id, &config.antiemoji.punishment).await?;
                }
            }
        }
    }

    Ok(())
}

async fn apply_emoji_punishment(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    punishment: &PunishmentType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match punishment {
        PunishmentType::Ban => {
            guild_id.ban_with_reason(&ctx.http, user_id, 0, "Anti-Emoji: Unauthorized emoji modification").await?;
        }
        PunishmentType::Kick => {
            guild_id.kick_with_reason(&ctx.http, user_id, "Anti-Emoji: Unauthorized emoji modification").await?;
        }
        PunishmentType::StripRoles => {
            if let Ok(member) = guild_id.member(&ctx.http, user_id).await {
                for role_id in &member.roles {
                    let _ = guild_id.member(&ctx.http, user_id).await?.remove_role(&ctx.http, role_id).await;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
