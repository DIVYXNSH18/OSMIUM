use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::logging::logger::Logger;
use tracing::error;

pub async fn handle(ctx: &Context, guild_id: GuildId, user: &User, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if let Some(log_channel) = config.log_channel_id {
        let logger = Logger::new(ctx, &log_channel);
        if let Err(e) = logger.log_member_leave(user, guild_id).await {
            error!("Failed to log member leave: {}", e);
        }
    }

    Ok(())
}
