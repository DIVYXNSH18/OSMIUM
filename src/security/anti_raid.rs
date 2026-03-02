use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::collections::VecDeque;
use std::time::Instant;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, PunishmentType};
use crate::logging::logger::Logger;
use tracing::{info, error};
use chrono::Utc;

pub async fn check_raid(
    ctx: &Context,
    guild_id: GuildId,
    member: &Member,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut joins = data.raid_joins.entry(guild_id).or_insert_with(VecDeque::new);
    let now = Instant::now();
    
    joins.retain(|t| t.elapsed().as_millis() < config.antiraid.join_row_ms as u128);
    joins.push_back(now);

    if joins.len() >= config.antiraid.join_row_threshold as usize {
        info!("Raid detected in guild {} - {} joins in {}ms", guild_id, joins.len(), config.antiraid.join_row_ms);
        activate_anti_raid(ctx, guild_id, config, data.clone()).await?;
        return Ok(());
    }

    let risk_score = calculate_risk_score(member).await;
    
    let mut conn = data.redis_pool.get().await?;
    let key = format!("raid_score:{}", guild_id);
    let current_score: u32 = redis::cmd("GET")
        .arg(&key)
        .query_async(&mut *conn)
        .await
        .unwrap_or(0);

    let new_score = current_score + risk_score;
    
    redis::cmd("SET")
        .arg(&key)
        .arg(new_score)
        .arg("EX")
        .arg(config.antiraid.score_reset_minutes * 60)
        .query_async(&mut *conn)
        .await?;

    if new_score >= config.antiraid.score_limit {
        info!("Raid score threshold exceeded in guild {}: {}/{}", guild_id, new_score, config.antiraid.score_limit);
        activate_anti_raid(ctx, guild_id, config, data).await?;
    }

    Ok(())
}

async fn calculate_risk_score(member: &Member) -> u32 {
    let mut score = 0;

    let account_age = Utc::now().timestamp() - member.user.created_at().unix_timestamp();
    let days_old = account_age / 86400;

    if days_old < 1 {
        score += 20;
    } else if days_old < 7 {
        score += 10;
    } else if days_old < 30 {
        score += 5;
    }

    if member.user.avatar.is_none() {
        score += 10;
    }

    if member.premium_since.is_none() {
        score += 5;
    }

    let username = &member.user.name;
    if username.chars().filter(|c| c.is_numeric()).count() > username.len() / 2 {
        score += 10;
    }

    score
}

async fn activate_anti_raid(
    ctx: &Context,
    guild_id: GuildId,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::{CreateMessage, CreateEmbed};
    
    if config.antiraid.lock_channels {
        lock_all_channels(ctx, guild_id).await?;
    }

    punish_recent_joiners(ctx, guild_id, &config.antiraid.punishment).await?;

    if let Some(log_channel) = &config.log_channel_id {
        let logger = Logger::new(ctx, log_channel);
        logger.log_raid_detected(guild_id).await?;
    }

    if let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await {
        if let Ok(owner) = guild.owner_id.to_user(&ctx.http).await {
            let embed = CreateEmbed::new()
                .title("🚨 Raid Detected")
                .description(format!("A raid has been detected in **{}**. Anti-raid measures have been activated.", guild.name))
                .color(0xFF0000)
                .timestamp(Utc::now());
            let message = CreateMessage::new().embed(embed);
            let _ = owner.direct_message(&ctx.http, message).await;
        }
    }

    Ok(())
}

async fn lock_all_channels(ctx: &Context, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::EditChannel;
    
    if let Ok(channels) = guild_id.channels(&ctx.http).await {
        for (_, channel) in channels {
            if channel.kind == ChannelType::Text {
                let overwrite = PermissionOverwrite {
                    allow: Permissions::empty(),
                    deny: Permissions::SEND_MESSAGES,
                    kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
                };
                let builder = EditChannel::new().permissions(vec![overwrite]);
                let _ = channel.edit(&ctx.http, builder).await;
            }
        }
    }
    Ok(())
}

async fn punish_recent_joiners(
    ctx: &Context,
    guild_id: GuildId,
    punishment: &PunishmentType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Ok(members) = guild_id.members(&ctx.http, None, None).await {
        let recent_threshold = Utc::now().timestamp() - 300;
        
        for member in members {
            if let Some(joined_at) = member.joined_at {
                if joined_at.unix_timestamp() > recent_threshold {
                    match punishment {
                        PunishmentType::Ban => {
                            let _ = guild_id.ban(&ctx.http, member.user.id, 0).await;
                        }
                        PunishmentType::Kick => {
                            let _ = guild_id.kick(&ctx.http, member.user.id).await;
                        }
                        PunishmentType::Timeout => {
                            let _ = guild_id.member(&ctx.http, member.user.id).await.ok().and_then(|mut m| {
                                futures::executor::block_on(async {
                                    m.disable_communication_until_datetime(&ctx.http, serenity::model::Timestamp::from_unix_timestamp(Utc::now().timestamp() + 3600).unwrap()).await.ok()
                                })
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
