use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only, subcommands("add", "remove", "list"))]
pub async fn whitelist(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "User to whitelist"] user: Option<serenity::User>,
    #[description = "Role to whitelist"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    if let Some(user) = user {
        if !config.antinuke.whitelisted_users.contains(&user.id.to_string()) {
            config.antinuke.whitelisted_users.push(user.id.to_string());
            ctx.data().database.queries.update_guild_config(&config).await?;
            ctx.say(format!("✅ Added {} to whitelist", user.tag())).await?;
        } else {
            ctx.say("User is already whitelisted").await?;
        }
    } else if let Some(role) = role {
        if !config.antinuke.whitelisted_roles.contains(&role.id.to_string()) {
            config.antinuke.whitelisted_roles.push(role.id.to_string());
            ctx.data().database.queries.update_guild_config(&config).await?;
            ctx.say(format!("✅ Added {} to whitelist", role.name)).await?;
        } else {
            ctx.say("Role is already whitelisted").await?;
        }
    } else {
        ctx.say("Please specify a user or role").await?;
    }

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "User to remove from whitelist"] user: Option<serenity::User>,
    #[description = "Role to remove from whitelist"] role: Option<serenity::Role>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let mut config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    if let Some(user) = user {
        config.antinuke.whitelisted_users.retain(|id| id != &user.id.to_string());
        ctx.data().database.queries.update_guild_config(&config).await?;
        ctx.say(format!("✅ Removed {} from whitelist", user.tag())).await?;
    } else if let Some(role) = role {
        config.antinuke.whitelisted_roles.retain(|id| id != &role.id.to_string());
        ctx.data().database.queries.update_guild_config(&config).await?;
        ctx.say(format!("✅ Removed {} from whitelist", role.name)).await?;
    } else {
        ctx.say("Please specify a user or role").await?;
    }

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let config = ctx.data().database.queries.get_guild_config(&guild_id.to_string()).await?;
    
    let mut response = "**Whitelisted Users:**\n".to_string();
    for user_id in &config.antinuke.whitelisted_users {
        response.push_str(&format!("<@{}>\n", user_id));
    }
    
    response.push_str("\n**Whitelisted Roles:**\n");
    for role_id in &config.antinuke.whitelisted_roles {
        response.push_str(&format!("<@&{}>\n", role_id));
    }

    ctx.say(response).await?;
    Ok(())
}
