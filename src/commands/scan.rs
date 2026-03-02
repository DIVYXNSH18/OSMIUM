use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only)]
pub async fn scan(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    ctx.defer().await?;

    let result = crate::scanner::server_scan::scan_server(ctx.serenity_context(), guild_id).await?;

    let mut response = format!("**🔍 Security Scan Results**\n\n**Overall Score: {}/100**\n\n", result.score);

    for finding in result.findings {
        response.push_str(&format!("{} **{}**\n{}\n\n", finding.severity.emoji(), finding.title, finding.description));
    }

    ctx.say(response).await?;
    Ok(())
}
