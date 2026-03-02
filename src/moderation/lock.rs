use serenity::{client::Context, model::prelude::*};
use serenity::builder::EditChannel;
use crate::database::models::{ModAction, ModerationLog};
use crate::database::Database;
use chrono::Utc;

pub async fn lock_channel(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
    moderator_id: UserId,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let channel = channel_id.to_channel(&ctx.http).await?;
    
    if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
        let overwrite = PermissionOverwrite {
            allow: Permissions::empty(),
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        };
        let builder = EditChannel::new().permissions(vec![overwrite]);
        guild_channel.edit(&ctx.http, builder).await?;
    }

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Lock,
        moderator_id: moderator_id.to_string(),
        target_id: channel_id.to_string(),
        reason: None,
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}

pub async fn unlock_channel(
    ctx: &Context,
    channel_id: ChannelId,
    guild_id: GuildId,
    moderator_id: UserId,
    database: &Database,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let channel = channel_id.to_channel(&ctx.http).await?;
    
    if let serenity::model::channel::Channel::Guild(guild_channel) = channel {
        let overwrite = PermissionOverwrite {
            allow: Permissions::SEND_MESSAGES,
            deny: Permissions::empty(),
            kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
        };
        let builder = EditChannel::new().permissions(vec![overwrite]);
        guild_channel.edit(&ctx.http, builder).await?;
    }

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Unlock,
        moderator_id: moderator_id.to_string(),
        target_id: channel_id.to_string(),
        reason: None,
        timestamp: Utc::now(),
    }).await?;

    Ok(())
}

pub async fn lock_all_channels(
    ctx: &Context,
    guild_id: GuildId,
    moderator_id: UserId,
    database: &Database,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let channels = guild_id.channels(&ctx.http).await?;
    let mut count = 0;

    for (_, channel) in channels {
        if channel.kind == ChannelType::Text {
            let overwrite = PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::SEND_MESSAGES,
                kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
            };
            let builder = EditChannel::new().permissions(vec![overwrite]);
            let _ = channel.edit(&ctx.http, builder).await;
            count += 1;
        }
    }

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Lock,
        moderator_id: moderator_id.to_string(),
        target_id: "all_channels".to_string(),
        reason: Some(format!("Locked {} channels", count)),
        timestamp: Utc::now(),
    }).await?;

    Ok(count)
}

pub async fn unlock_all_channels(
    ctx: &Context,
    guild_id: GuildId,
    moderator_id: UserId,
    database: &Database,
) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
    let channels = guild_id.channels(&ctx.http).await?;
    let mut count = 0;

    for (_, channel) in channels {
        if channel.kind == ChannelType::Text {
            let overwrite = PermissionOverwrite {
                allow: Permissions::SEND_MESSAGES,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Role(guild_id.everyone_role()),
            };
            let builder = EditChannel::new().permissions(vec![overwrite]);
            let _ = channel.edit(&ctx.http, builder).await;
            count += 1;
        }
    }

    database.queries.add_moderation_log(ModerationLog {
        guild_id: guild_id.to_string(),
        action: ModAction::Unlock,
        moderator_id: moderator_id.to_string(),
        target_id: "all_channels".to_string(),
        reason: Some(format!("Unlocked {} channels", count)),
        timestamp: Utc::now(),
    }).await?;

    Ok(count)
}
