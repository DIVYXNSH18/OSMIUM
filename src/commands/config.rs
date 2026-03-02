use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only)]
pub async fn setlogchannel(
    ctx: Context<'_>,
    #[description = "Log channel"] channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    config.log_channel_id = Some(channel.id.to_string());
    ctx.data().database.queries.update_guild_config(&config).await?;
    
    ctx.say(format!("✅ Log channel set to <#{}>", channel.id)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    let response = format!(
        "**Server Configuration**\n\n\
        **Anti-Nuke:** {}\n\
        **Anti-Raid:** {}\n\
        **Anti-Spam:** {}\n\
        **Anti-Mention:** {}\n\
        **Anti-Ghost-Ping:** {}\n\
        **Anti-Vanity:** {}\n\
        **Anti-Rename:** {}\n\
        **Anti-Emoji:** {}\n\
        **Anti-Invite:** {}\n\
        **Verification:** {}\n\
        **Log Channel:** {}\n",
        if config.antinuke.enabled { "✅" } else { "❌" },
        if config.antiraid.enabled { "✅" } else { "❌" },
        if config.antispam.enabled { "✅" } else { "❌" },
        if config.antimention.enabled { "✅" } else { "❌" },
        if config.antighost.enabled { "✅" } else { "❌" },
        if config.antivanity.enabled { "✅" } else { "❌" },
        if config.antirename.enabled { "✅" } else { "❌" },
        if config.antiemoji.enabled { "✅" } else { "❌" },
        if config.antiinvite.enabled { "✅" } else { "❌" },
        if config.verification.enabled { "✅" } else { "❌" },
        config.log_channel_id.as_ref().map(|id| format!("<#{}>", id)).unwrap_or_else(|| "Not set".to_string())
    );
    
    ctx.say(response).await?;
    Ok(())
}
