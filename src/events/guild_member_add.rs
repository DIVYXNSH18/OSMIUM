use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::{anti_raid, anti_nuke};
use crate::verification;
use crate::logging::logger::Logger;
use tracing::error;

pub async fn handle(ctx: &Context, member: &Member, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = member.guild_id;
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if config.antiraid.enabled {
        if let Err(e) = anti_raid::check_raid(ctx, guild_id, member, &config, data.clone()).await {
            error!("Anti-raid check failed: {}", e);
        }
    }

    if config.verification.enabled {
        if let Err(e) = verification::manager::handle_new_member(ctx, member, &config, data.clone()).await {
            error!("Verification handling failed: {}", e);
        }
    }

    if let Some(log_channel) = config.log_channel_id {
        let logger = Logger::new(ctx, &log_channel);
        if let Err(e) = logger.log_member_join(member).await {
            error!("Failed to log member join: {}", e);
        }
    }

    Ok(())
}
