use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::whitelist;
use crate::logging::logger::Logger;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

pub async fn check_invite_delete(
    ctx: &Context,
    guild_id: GuildId,
    event: &InviteDeleteEvent,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::InviteDelete), None, None, Some(1)).await {
        if let Some(entry) = audit_logs.entries.values().next() {
            if let Some(executor_id) = entry.user_id {
                let member = guild_id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    info!("User {} is whitelisted, skipping anti-invite punishment", executor_id);
                    return Ok(());
                }

                info!("Unauthorized invite deletion by {} in guild {}", executor_id, guild_id);

                match config.antiinvite.punishment {
                    PunishmentType::Ban => {
                        guild_id.ban_with_reason(&ctx.http, executor_id, 0, "Anti-Invite: Unauthorized invite deletion").await?;
                    }
                    PunishmentType::Kick => {
                        guild_id.kick_with_reason(&ctx.http, executor_id, "Anti-Invite: Unauthorized invite deletion").await?;
                    }
                    PunishmentType::StripRoles => {
                        if let Some(member) = member {
                            for role_id in &member.roles {
                                let _ = guild_id.member(&ctx.http, executor_id).await?.remove_role(&ctx.http, role_id).await;
                            }
                        }
                    }
                    _ => {}
                }

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_invite_delete(executor_id, &event.code, event.channel_id, &config.antiinvite.punishment).await?;
                }
            }
        }
    }

    Ok(())
}
