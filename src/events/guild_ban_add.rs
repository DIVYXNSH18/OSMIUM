use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::time::Instant;
use crate::bot::data::BotData;
use crate::security::anti_nuke::{self, NukeAction};
use crate::logging::logger::Logger;
use tracing::error;
use tokio::time::{sleep, Duration};

pub async fn handle(ctx: &Context, guild_id: GuildId, banned_user: &User, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    sleep(Duration::from_millis(500)).await;

    if config.antinuke.enabled {
        if let Ok(audit_logs) = guild_id.audit_logs(&ctx.http, Some(serenity::model::guild::audit_log::Action::MemberBanAdd), None, None, Some(1)).await {
            if let Some(entry) = audit_logs.entries.values().next() {
                if let Some(executor_id) = entry.user_id {
                    let key = (guild_id, executor_id);
                    let mut tracker = data.nuke_tracker.entry(key).or_insert_with(Vec::new);
                    tracker.retain(|t| t.elapsed().as_secs() < 10);
                    tracker.push(Instant::now());

                    if let Err(e) = anti_nuke::check_threshold(
                        ctx,
                        guild_id,
                        executor_id,
                        NukeAction::MassBan,
                        tracker.len() as u32,
                        &config,
                        data.clone(),
                    ).await {
                        error!("Anti-nuke check failed: {}", e);
                    }
                }
            }
        }
    }

    if let Some(log_channel) = config.log_channel_id {
        let logger = Logger::new(ctx, &log_channel);
        if let Err(e) = logger.log_member_ban(banned_user, guild_id, None).await {
            error!("Failed to log ban: {}", e);
        }
    }

    Ok(())
}
