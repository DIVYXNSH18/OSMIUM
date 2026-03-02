use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::anti_rename;
use tracing::error;

pub async fn handle(ctx: &Context, old: Option<Channel>, new: Channel, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let (Some(Channel::Guild(old_channel)), Channel::Guild(new_channel)) = (old, new) {
        let guild_id = new_channel.guild_id;
        let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

        if config.antirename.enabled && config.antirename.channel_rename {
            if old_channel.name != new_channel.name {
                if let Err(e) = anti_rename::check_channel_rename(ctx, &old_channel, &new_channel, &config, data.clone()).await {
                    error!("Anti-rename check failed: {}", e);
                }
            }
        }
    }

    Ok(())
}
