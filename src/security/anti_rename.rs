use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::whitelist;
use crate::logging::logger::Logger;
use tracing::{info, error};
use tokio::time::{sleep, Duration};

pub async fn check_server_rename(
    ctx: &Context,
    old_guild: &Guild,
    new_guild: &PartialGuild,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::EditGuild;
    
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = new_guild.id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::GuildUpdate), None, None, Some(1)).await {
        if let Some(entry) = audit_logs.entries.values().next() {
            if let Some(executor_id) = entry.user_id {
                let member = new_guild.id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    return Ok(());
                }

                info!("Unauthorized server rename by {} in guild {}", executor_id, new_guild.id);

                let builder = EditGuild::new().name(&old_guild.name);
                if let Err(e) = new_guild.id.edit(&ctx.http, builder).await {
                    error!("Failed to revert server name: {}", e);
                }

                apply_rename_punishment(ctx, new_guild.id, executor_id, &config.antirename.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_server_rename(executor_id, &old_guild.name, &new_guild.name, &config.antirename.punishment).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn check_server_icon(
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
                    return Ok(());
                }

                info!("Unauthorized server icon change by {} in guild {}", executor_id, new_guild.id);

                apply_rename_punishment(ctx, new_guild.id, executor_id, &config.antirename.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_server_icon_change(executor_id, &config.antirename.punishment).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn check_role_rename(
    ctx: &Context,
    old_role: &Role,
    new_role: &Role,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::EditRole;
    
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = new_role.guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::RoleUpdate), None, None, Some(1)).await {
        if let Some(entry) = audit_logs.entries.values().next() {
            if let Some(executor_id) = entry.user_id {
                let member = new_role.guild_id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    return Ok(());
                }

                info!("Unauthorized role rename by {} in guild {}", executor_id, new_role.guild_id);

                let builder = EditRole::new().name(&old_role.name);
                if let Err(e) = new_role.guild_id.edit_role(&ctx.http, new_role.id, builder).await {
                    error!("Failed to revert role name: {}", e);
                }

                apply_rename_punishment(ctx, new_role.guild_id, executor_id, &config.antirename.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_role_rename(executor_id, &old_role.name, &new_role.name, &config.antirename.punishment).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn check_channel_rename(
    ctx: &Context,
    old_channel: &GuildChannel,
    new_channel: &GuildChannel,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::EditChannel;
    
    sleep(Duration::from_millis(500)).await;

    if let Ok(audit_logs) = new_channel.guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::ChannelUpdate), None, None, Some(1)).await {
        if let Some(entry) = audit_logs.entries.values().next() {
            if let Some(executor_id) = entry.user_id {
                let member = new_channel.guild_id.member(&ctx.http, executor_id).await.ok();
                
                if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
                    return Ok(());
                }

                info!("Unauthorized channel rename by {} in guild {}", executor_id, new_channel.guild_id);

                let builder = EditChannel::new().name(&old_channel.name);
                if let Err(e) = new_channel.edit(&ctx.http, builder).await {
                    error!("Failed to revert channel name: {}", e);
                }

                apply_rename_punishment(ctx, new_channel.guild_id, executor_id, &config.antirename.punishment).await?;

                if let Some(log_channel) = &config.log_channel_id {
                    let logger = Logger::new(ctx, log_channel);
                    logger.log_channel_rename(executor_id, &old_channel.name, &new_channel.name, &config.antirename.punishment).await?;
                }
            }
        }
    }

    Ok(())
}

async fn apply_rename_punishment(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    punishment: &PunishmentType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match punishment {
        PunishmentType::Ban => {
            guild_id.ban_with_reason(&ctx.http, user_id, 0, "Anti-Rename: Unauthorized rename").await?;
        }
        PunishmentType::Kick => {
            guild_id.kick_with_reason(&ctx.http, user_id, "Anti-Rename: Unauthorized rename").await?;
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
