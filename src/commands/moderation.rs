use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, crate::bot::data::BotData, Error>;

#[poise::command(slash_command, guild_only)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "User to ban"] user: serenity::User,
    #[description = "Reason for ban"] reason: Option<String>,
    #[description = "Days of messages to delete"] delete_days: Option<u8>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::ban::ban_user(
        ctx.serenity_context(),
        guild_id,
        user.id,
        ctx.author().id,
        reason.clone(),
        delete_days.unwrap_or(0),
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Banned {} - {}", user.tag(), reason.unwrap_or_else(|| "No reason".to_string()))).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
    #[description = "Reason for kick"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::kick::kick_user(
        ctx.serenity_context(),
        guild_id,
        user.id,
        ctx.author().id,
        reason.clone(),
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Kicked {} - {}", user.tag(), reason.unwrap_or_else(|| "No reason".to_string()))).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn mute(
    ctx: Context<'_>,
    #[description = "User to mute"] user: serenity::User,
    #[description = "Duration in minutes"] duration: i64,
    #[description = "Reason for mute"] reason: Option<String>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::mute::mute_user(
        ctx.serenity_context(),
        guild_id,
        user.id,
        ctx.author().id,
        duration * 60,
        reason.clone(),
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Muted {} for {} minutes", user.tag(), duration)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn unmute(
    ctx: Context<'_>,
    #[description = "User to unmute"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::mute::unmute_user(
        ctx.serenity_context(),
        guild_id,
        user.id,
        ctx.author().id,
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Unmuted {}", user.tag())).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn warn(
    ctx: Context<'_>,
    #[description = "User to warn"] user: serenity::User,
    #[description = "Reason for warning"] reason: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::warn::warn_user(
        ctx.serenity_context(),
        guild_id,
        user.id,
        ctx.author().id,
        reason.clone(),
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Warned {} - {}", user.tag(), reason)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn warnings(
    ctx: Context<'_>,
    #[description = "User to check warnings"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    let warnings = crate::moderation::warn::get_warnings(guild_id, user.id, &ctx.data().database).await?;

    if warnings.is_empty() {
        ctx.say(format!("{} has no warnings", user.tag())).await?;
    } else {
        let mut response = format!("**Warnings for {}:**\n", user.tag());
        for (i, warning) in warnings.iter().enumerate() {
            response.push_str(&format!("{}. {} - <t:{}:R>\n", i + 1, warning.reason, warning.timestamp.timestamp()));
        }
        ctx.say(response).await?;
    }

    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn clearwarnings(
    ctx: Context<'_>,
    #[description = "User to clear warnings"] user: serenity::User,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    crate::moderation::warn::clear_warnings(guild_id, user.id, &ctx.data().database).await?;

    ctx.say(format!("✅ Cleared all warnings for {}", user.tag())).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn clear(
    ctx: Context<'_>,
    #[description = "Number of messages to delete"] amount: u64,
    #[description = "Only delete messages from this user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let channel_id = ctx.channel_id();
    
    let count = crate::moderation::clear::clear_messages(
        ctx.serenity_context(),
        channel_id,
        guild_id,
        ctx.author().id,
        amount.min(100),
        user.as_ref().map(|u| u.id),
        &ctx.data().database,
    ).await?;

    ctx.say(format!("✅ Deleted {} messages", count)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn lock(
    ctx: Context<'_>,
    #[description = "Channel to lock"] channel: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let channel_id = channel.as_ref().map(|c| c.id).unwrap_or(ctx.channel_id());
    
    crate::moderation::lock::lock_channel(
        ctx.serenity_context(),
        channel_id,
        guild_id,
        ctx.author().id,
        &ctx.data().database,
    ).await?;

    ctx.say(format!("🔒 Locked <#{}>", channel_id)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn unlock(
    ctx: Context<'_>,
    #[description = "Channel to unlock"] channel: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    let channel_id = channel.as_ref().map(|c| c.id).unwrap_or(ctx.channel_id());
    
    crate::moderation::lock::unlock_channel(
        ctx.serenity_context(),
        channel_id,
        guild_id,
        ctx.author().id,
        &ctx.data().database,
    ).await?;

    ctx.say(format!("🔓 Unlocked <#{}>", channel_id)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn lockall(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    let count = crate::moderation::lock::lock_all_channels(
        ctx.serenity_context(),
        guild_id,
        ctx.author().id,
        &ctx.data().database,
    ).await?;

    ctx.say(format!("🔒 Locked {} channels", count)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn unlockall(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    let count = crate::moderation::lock::unlock_all_channels(
        ctx.serenity_context(),
        guild_id,
        ctx.author().id,
        &ctx.data().database,
    ).await?;

    ctx.say(format!("🔓 Unlocked {} channels", count)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn slowmode(
    ctx: Context<'_>,
    #[description = "Slowmode duration in seconds"] seconds: u64,
    #[description = "Channel to apply slowmode"] channel: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    use serenity::builder::EditChannel;
    
    let channel_id = channel.as_ref().map(|c| c.id).unwrap_or(ctx.channel_id());
    
    let channel = channel_id.to_channel(ctx.serenity_context()).await?;
    if let serenity::Channel::Guild(guild_channel) = channel {
        let builder = EditChannel::new().rate_limit_per_user(seconds as u16);
        guild_channel.edit(ctx.serenity_context(), builder).await?;
    }

    ctx.say(format!("⏱️ Set slowmode to {} seconds in <#{}>", seconds, channel_id)).await?;
    Ok(())
}

#[poise::command(slash_command, guild_only)]
pub async fn nickname(
    ctx: Context<'_>,
    #[description = "User to change nickname"] user: serenity::User,
    #[description = "New nickname"] nickname: String,
) -> Result<(), Error> {
    use serenity::builder::EditMember;
    
    let guild_id = ctx.guild_id().ok_or("Not in a guild")?;
    
    let mut member = guild_id.member(ctx.serenity_context(), user.id).await?;
    let builder = EditMember::new().nickname(&nickname);
    member.edit(ctx.serenity_context(), builder).await?;

    ctx.say(format!("✅ Changed nickname of {} to {}", user.tag(), nickname)).await?;
    Ok(())
}
