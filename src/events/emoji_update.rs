use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::collections::HashMap;
use crate::bot::data::BotData;
use crate::security::anti_emoji;
use tracing::error;

pub async fn handle(
    ctx: &Context,
    guild_id: GuildId,
    current_state: HashMap<EmojiId, Emoji>,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if config.antiemoji.enabled {
        if let Err(e) = anti_emoji::check_emoji_changes(ctx, guild_id, current_state, &config, data.clone()).await {
            error!("Anti-emoji check failed: {}", e);
        }
    }

    Ok(())
}
