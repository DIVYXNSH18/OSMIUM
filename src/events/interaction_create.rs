use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::verification::button;
use tracing::error;

pub async fn handle(ctx: &Context, interaction: Interaction, data: Arc<BotData>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Interaction::MessageComponent(component) = interaction {
        if component.data.custom_id.starts_with("verify_") {
            if let Err(e) = button::handle_verification_button(ctx, &component, data.clone()).await {
                error!("Verification button handling failed: {}", e);
            }
        }
    }

    Ok(())
}
