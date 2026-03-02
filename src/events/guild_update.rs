use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::{anti_rename, anti_vanity};
use tracing::error;

pub async fn handle(ctx: &Context, old: Option<Guild>, new: PartialGuild, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = new.id;
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if let Some(old_guild) = old {
        if config.antirename.enabled && config.antirename.server_rename && old_guild.name != new.name {
            if let Err(e) = anti_rename::check_server_rename(ctx, &old_guild, &new, &config, data.clone()).await {
                error!("Anti-rename (server name) check failed: {}", e);
            }
        }

        if config.antirename.enabled && config.antirename.server_icon {
            if old_guild.icon != new.icon {
                if let Err(e) = anti_rename::check_server_icon(ctx, &old_guild, &new, &config, data.clone()).await {
                    error!("Anti-rename (server icon) check failed: {}", e);
                }
            }
        }

        if config.antivanity.enabled {
            if old_guild.vanity_url_code != new.vanity_url_code {
                if let Err(e) = anti_vanity::check_vanity_change(ctx, &old_guild, &new, &config, data.clone()).await {
                    error!("Anti-vanity check failed: {}", e);
                }
            }
        }
    }

    Ok(())
}
