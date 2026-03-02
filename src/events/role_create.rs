use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::logging::logger::Logger;
use tracing::error;

pub async fn handle(ctx: &Context, role: &Role, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = role.guild_id;
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if let Some(log_channel) = config.log_channel_id {
        let logger = Logger::new(ctx, &log_channel);
        if let Err(e) = logger.log_role_create(role).await {
            error!("Failed to log role create: {}", e);
        }
    }

    Ok(())
}
