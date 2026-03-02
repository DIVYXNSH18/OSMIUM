use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::logging::logger::Logger;
use tracing::error;

pub async fn handle(ctx: &Context, channel: &GuildChannel, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = channel.guild_id;
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if let Some(log_channel) = config.log_channel_id {
        let logger = Logger::new(ctx, &log_channel);
        if let Err(e) = logger.log_channel_create(channel).await {
            error!("Failed to log channel create: {}", e);
        }
    }

    Ok(())
}
