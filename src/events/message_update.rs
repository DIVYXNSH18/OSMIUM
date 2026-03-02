use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::logging::logger::Logger;
use tracing::error;

pub async fn handle(
    ctx: &Context,
    old: Option<Message>,
    new: Option<Message>,
    _event: MessageUpdateEvent,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let (Some(old_msg), Some(new_msg)) = (old, new) {
        if old_msg.content == new_msg.content || new_msg.author.bot {
            return Ok(());
        }

        if let Some(guild_id) = new_msg.guild_id {
            let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;
            
            if let Some(log_channel) = config.log_channel_id {
                let logger = Logger::new(ctx, &log_channel);
                if let Err(e) = logger.log_message_edit(&old_msg, &new_msg).await {
                    error!("Failed to log message edit: {}", e);
                }
            }
        }
    }

    Ok(())
}
