use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::whitelist;
use crate::logging::logger::Logger;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

pub async fn check_vanity_change(
    ctx: &Context,
    old_guild: &Guild,
    new_guild: &PartialGuild,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = new_guild.id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::GuildUpdate), None, None, Some(1)).await {
        if let Some(entry) = audit_logs.entries.values().next() {
            if let Some(executor_id) = entry.user_id {
                let member = new_guild.id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    info!("User {} is whitelisted, skipping anti-vanity punishment", executor_id);
                    return Ok(());
                }

                info!("Unauthorized vanity change by {} in guild {}", executor_id, new_guild.id);

                if let Some(old_vanity) = &old_guild.vanity_url_code {
                    info!("Attempting to revert vanity URL to: {}", old_vanity);
                }

                match config.antivanity.punishment {
                    PunishmentType::Ban => {
                        new_guild.id.ban_with_reason(&ctx.http, executor_id, 0, "Anti-Vanity: Unauthorized vanity URL change").await?;
                    }
                    PunishmentType::Kick => {
                        new_guild.id.kick_with_reason(&ctx.http, executor_id, "Anti-Vanity: Unauthorized vanity URL change").await?;
                    }
                    PunishmentType::StripRoles => {
                        if let Some(member) = member {
                            for role_id in &member.roles {
                                let _ = new_guild.id.member(&ctx.http, executor_id).await?.remove_role(&ctx.http, role_id).await;
                            }
                        }
                    }
                    _ => {}
                }

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_vanity_change(
                        executor_id,
                        old_guild.vanity_url_code.as_deref(),
                        new_guild.vanity_url_code.as_deref(),
                        &config.antivanity.punishment,
                    ).await?;
                }
            }
        }
    }

    Ok(())
}
