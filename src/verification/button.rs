use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use tracing::{info, error};

pub async fn send_verification_message(
    ctx: &Context,
    channel_id: ChannelId,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::*;
    
    let button = CreateButton::new("verify_button")
        .label("Verify")
        .style(serenity::model::application::component::ButtonStyle::Success);
    
    let action_row = CreateActionRow::Buttons(vec![button]);
    
    let embed = CreateEmbed::new()
        .title("✅ Verification Required")
        .description("Click the button below to verify yourself and gain access to the server.")
        .color(0x00FF00);
    
    let message = CreateMessage::new()
        .embed(embed)
        .components(vec![action_row]);
    
    channel_id.send_message(&ctx.http, message).await?;

    Ok(())
}

pub async fn handle_verification_button(
    ctx: &Context,
    interaction: &serenity::model::application::interaction::MessageComponentInteraction,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::*;
    
    let guild_id = interaction.guild_id.ok_or("No guild ID")?;
    let user_id = interaction.user.id;

    let config = data.database.queries.get_guild_config(&guild_id.to_string()).await?;

    if !config.verification.enabled {
        return Ok(());
    }

    let mut member = guild_id.member(&ctx.http, user_id).await?;

    if let Some(unverified_role_id) = &config.verification.unverified_role_id {
        if let Ok(role_id) = unverified_role_id.parse::<u64>() {
            let _ = member.remove_role(&ctx.http, RoleId::new(role_id)).await;
        }
    }

    if let Some(verified_role_id) = &config.verification.verified_role_id {
        if let Ok(role_id) = verified_role_id.parse::<u64>() {
            member.add_role(&ctx.http, RoleId::new(role_id)).await?;
        }
    }

    let response_data = CreateInteractionResponseMessage::new()
        .content("✅ You have been verified successfully!")
        .ephemeral(true);
    
    let response = CreateInteractionResponse::Message(response_data);
    
    interaction.create_response(&ctx.http, response).await?;

    info!("User {} verified in guild {}", user_id, guild_id);

    Ok(())
}
