use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::time::Instant;
use crate::bot::data::{BotData, BeastModeState};
use crate::logging::logger::Logger;
use tracing::info;

pub async fn activate(
    ctx: &Context,
    guild_id: GuildId,
    activated_by: String,
    log_channel: Option<&str>,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let state = BeastModeState {
        activated_at: Instant::now(),
        activated_by: activated_by.clone(),
    };

    data.beast_mode.insert(guild_id, state);
    info!("Beast Mode activated for guild {} by {}", guild_id, activated_by);

    if let Some(log_channel) = log_channel {
        let logger = Logger::new(ctx, log_channel);
        logger.log_beast_mode_activated(guild_id, &activated_by).await?;
    }

    Ok(())
}

pub fn is_active(guild_id: GuildId, data: Arc<BotData>) -> bool {
    if let Some(state) = data.beast_mode.get(&guild_id) {
        state.is_active()
    } else {
        false
    }
}

pub async fn deactivate(
    ctx: &Context,
    guild_id: GuildId,
    log_channel: Option<&str>,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    data.beast_mode.remove(&guild_id);
    info!("Beast Mode deactivated for guild {}", guild_id);

    if let Some(log_channel) = log_channel {
        let logger = Logger::new(ctx, log_channel);
        logger.log_beast_mode_deactivated(guild_id).await?;
    }

    Ok(())
}
