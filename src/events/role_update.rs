use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::security::anti_rename;
use tracing::error;

pub async fn handle(ctx: &Context, old: Option<Role>, new: Role, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let guild_id = new.guild_id;
    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if config.antirename.enabled && config.antirename.role_rename {
        if let Some(old_role) = old {
            if old_role.name != new.name {
                if let Err(e) = anti_rename::check_role_rename(ctx, &old_role, &new, &config, data.clone()).await {
                    error!("Anti-rename check failed: {}", e);
                }
            }
        }
    }

    Ok(())
}
