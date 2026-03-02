use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only)]
pub async fn setup_verification(
    ctx: Context<'_>,
    #[description = "Enable verification"] enabled: bool,
    #[description = "Verification method"] method: String,
    #[description = "Unverified role"] unverified_role: Option<serenity::Role>,
    #[description = "Verified role"] verified_role: Option<serenity::Role>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    config.verification.enabled = enabled;
    config.verification.method = if method.to_lowercase() == "captcha" {
        crate::database::models::VerificationMethod::Captcha
    } else {
        crate::database::models::VerificationMethod::Button
    };
    
    if let Some(role) = unverified_role {
        config.verification.unverified_role_id = Some(role.id.to_string());
    }
    
    if let Some(role) = verified_role {
        config.verification.verified_role_id = Some(role.id.to_string());
    }
    
    ctx.data().database.queries.update_guild_config(&config).await?;
    
    ctx.say(format!("✅ Verification {} with method: {}", if enabled { "enabled" } else { "disabled" }, method)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn send_verify_button(
    ctx: Context<'_>,
    #[description = "Channel to send verification message"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    crate::verification::button::send_verification_message(ctx.serenity_context(), channel.id).await?;
    ctx.say("✅ Verification message sent").await?;
    Ok(())
}
