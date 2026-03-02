use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::anti_invite;
use tracing::error;

pub async fn handle(ctx: &Context, data_event: InviteDeleteEvent, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(guild_id) = data_event.guild_id {
        let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

        if config.antiinvite.enabled {
            if let Err(e) = anti_invite::check_invite_delete(ctx, guild_id, &data_event, &config, data.clone()).await {
                error!("Anti-invite check failed: {}", e);
            }
        }
    }

    Ok(())
}
