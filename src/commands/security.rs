use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only, subcommands("antinuke", "antiraid", "antispam"))]
pub async fn security(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn antinuke(
    ctx: Context<'_>,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    config.antinuke.enabled = enabled;
    ctx.data().database.queries.update_guild_config(&config).await?;

    ctx.say(format!("✅ Anti-Nuke {}", if enabled { "enabled" } else { "disabled" })).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn antiraid(
    ctx: Context<'_>,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    config.antiraid.enabled = enabled;
    ctx.data().database.queries.update_guild_config(&config).await?;

    ctx.say(format!("✅ Anti-Raid {}", if enabled { "enabled" } else { "disabled" })).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn antispam(
    ctx: Context<'_>,
    #[description = "Enable or disable"] enabled: bool,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    config.antispam.enabled = enabled;
    ctx.data().database.queries.update_guild_config(&config).await?;

    ctx.say(format!("✅ Anti-Spam {}", if enabled { "enabled" } else { "disabled" })).await?;
    Ok(())
}
