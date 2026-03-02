use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use std::time::Instant;
use crate::bot::data::{BotData, CachedMessage};
use crate::security::{anti_spam, anti_mention};
use tracing::error;

pub async fn handle(ctx: &Context, msg: &Message, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if msg.author.bot {
        return Ok(());
    }

    let Some(guild_id) = msg.guild_id else {
        return Ok(());
    };

    data.message_cache.insert(msg.id, CachedMessage {
        content: msg.content.clone(),
        author_id: msg.author.id,
        channel_id: msg.channel_id,
        mentions: msg.mentions.iter().map(|u| u.id).collect(),
        timestamp: Instant::now(),
    });

    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if config.antispam.enabled {
        if let Err(e) = anti_spam::check_spam(ctx, msg, &config, data.clone()).await {
            error!("Anti-spam check failed: {}", e);
        }
    }

    if config.antimention.enabled {
        if let Err(e) = anti_mention::check_mentions(ctx, msg, &config, data.clone()).await {
            error!("Anti-mention check failed: {}", e);
        }
    }

    Ok(())
}
