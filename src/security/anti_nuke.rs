use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::security::{whitelist, beast_mode};
use crate::logging::logger::Logger;
use tracing::{info, error};

#[derive(Debug, Clone, Copy)]
pub enum NukeAction {
    MassBan,
    MassKick,
    MassChannelDelete,
    MassRoleDelete,
    MassWebhookCreate,
    DangerousPermissionGrant,
}

impl NukeAction {
    fn threshold(&self, config: &GuildConfig) -> u32 {
        match self {
            Self::MassBan => config.antinuke.ban_threshold,
            Self::MassKick => config.antinuke.kick_threshold,
            Self::MassChannelDelete => config.antinuke.channel_delete_threshold,
            Self::MassRoleDelete => config.antinuke.role_delete_threshold,
            Self::MassWebhookCreate => config.antinuke.webhook_threshold,
            Self::DangerousPermissionGrant => 2,
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::MassBan => "Mass Ban",
            Self::MassKick => "Mass Kick",
            Self::MassChannelDelete => "Mass Channel Delete",
            Self::MassRoleDelete => "Mass Role Delete",
            Self::MassWebhookCreate => "Mass Webhook Create",
            Self::DangerousPermissionGrant => "Dangerous Permission Grant",
        }
    }
}

pub async fn check_threshold(
    ctx: &Context,
    guild_id: GuildId,
    executor_id: UserId,
    action: NukeAction,
    count: u32,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let threshold = action.threshold(config);

    if count < threshold {
        return Ok(());
    }

    let member = guild_id.member(&ctx.http, executor_id).await.ok();
    
    if whitelist::is_whitelisted(executor_id, member.as_ref(), config) {
        info!("User {} is whitelisted, skipping anti-nuke punishment", executor_id);
        return Ok(());
    }

    info!(
        "Anti-nuke triggered: {} by {} in guild {} (count: {}/{})",
        action.name(),
        executor_id,
        guild_id,
        count,
        threshold
    );

    if let Some(member) = &member {
        strip_all_roles(ctx, guild_id, member).await?;
    }

    apply_punishment(ctx, guild_id, executor_id, &config.antinuke.punishment).await?;

    if let Some(log_channel) = &config.log_channel_id {
        let logger = Logger::new(ctx, log_channel);
        logger.log_nuke_attempt(action.name(), executor_id, &config.antinuke.punishment, count).await?;
    }

    if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
        notify_owner(ctx, &guild, executor_id, action, count).await?;
    }

    if config.antinuke.beast_mode_enabled && count >= threshold + 2 {
        beast_mode::activate(ctx, guild_id, "Anti-Nuke System".to_string(), config.log_channel_id.as_deref(), data).await?;
    }

    Ok(())
}

async fn strip_all_roles(
    ctx: &Context,
    guild_id: GuildId,
    member: &Member,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for role_id in &member.roles {
        if let Err(e) = guild_id.member(&ctx.http, member.user.id).await?.remove_role(&ctx.http, role_id).await {
            error!("Failed to remove role {}: {}", role_id, e);
        }
    }
    Ok(())
}

async fn apply_punishment(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    punishment: &PunishmentType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match punishment {
        PunishmentType::Ban => {
            guild_id.ban_with_reason(&ctx.http, user_id, 0, "Anti-Nuke: Suspicious activity detected").await?;
        }
        PunishmentType::Kick => {
            guild_id.kick_with_reason(&ctx.http, user_id, "Anti-Nuke: Suspicious activity detected").await?;
        }
        PunishmentType::Timeout => {
            if let Ok(mut member) = guild_id.member(&ctx.http, user_id).await {
                let duration = chrono::Duration::hours(1);
                member.disable_communication_until_datetime(&ctx.http, serenity::model::Timestamp::from_unix_timestamp(chrono::Utc::now().timestamp() + duration.num_seconds()).unwrap()).await?;
            }
        }
        PunishmentType::StripRoles => {
        }
        PunishmentType::Warn => {
        }
    }
    Ok(())
}

async fn notify_owner(
    ctx: &Context,
    guild: &PartialGuild,
    executor_id: UserId,
    action: NukeAction,
    count: u32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::{CreateMessage, CreateEmbed};
    
    if let Ok(owner) = guild.owner_id.to_user(&ctx.http).await {
        let embed = CreateEmbed::new()
            .title("🚨 Anti-Nuke Alert")
            .description(format!(
                "**Server:** {}\n**Action:** {}\n**Executor:** <@{}>\n**Count:** {}\n\nImmediate action has been taken.",
                guild.name, action.name(), executor_id, count
            ))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        
        let message = CreateMessage::new().embed(embed);
        
        if let Err(e) = owner.direct_message(&ctx.http, message).await {
            error!("Failed to DM owner: {}", e);
        }
    }
    Ok(())
}
