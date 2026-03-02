use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::anti_ghost_ping;
use tracing::error;

pub async fn handle(
    ctx: &Context,
    channel_id: ChannelId,
    deleted_message_id: MessageId,
    guild_id: Option<GuildId>,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let Some(guild_id) = guild_id else {
        return Ok(());
    };

    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if config.antighost.enabled {
        if let Err(e) = anti_ghost_ping::check_ghost_ping(ctx, channel_id, deleted_message_id, guild_id, &config, data.clone()).await {
            error!("Anti-ghost-ping check failed: {}", e);
        }
    }

    Ok(())
}
