use serenity::{client::Context, model::prelude::*};
use std::sync::Arc;
use crate::bot::data::BotData;
use crate::database::models::{GuildConfig, VerificationMethod};
use crate::verification::captcha;
use tracing::{info, error};

pub async fn handle_new_member(
    ctx: &Context,
    member: &Member,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(unverified_role_id) = &config.verification.unverified_role_id {
        if let Ok(role_id) = unverified_role_id.parse::<u64>() {
            member.add_role(&ctx.http, RoleId::new(role_id)).await?;
        }
    }

    match config.verification.method {
        VerificationMethod::Captcha => {
            send_captcha_verification(ctx, member, config, data).await?;
        }
        VerificationMethod::Button => {
            if let Some(channel_id) = &config.verification.verification_channel_id {
                if let Ok(channel_id) = channel_id.parse::<u64>() {
                    use serenity::builder::CreateMessage;
                    let message = CreateMessage::new().content(format!("<@{}> Please verify yourself to access the server.", member.user.id));
                    let _ = ChannelId::new(channel_id).send_message(&ctx.http, message).await;
                }
            }
        }
    }

    Ok(())
}

async fn send_captcha_verification(
    ctx: &Context,
    member: &Member,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use serenity::builder::{CreateMessage, CreateAttachment};
    
    let (code, image_data) = captcha::generate_captcha(config.verification.captcha_length, config.verification.captcha_difficulty)?;

    data.database.queries.create_verification_attempt(
        &member.guild_id.to_string(),
        &member.user.id.to_string(),
        Some(code.clone()),
    ).await?;

    let attachment = CreateAttachment::bytes(image_data, "captcha.png");
    let message = CreateMessage::new()
        .content(format!(
            "Welcome to the server! Please solve this CAPTCHA within {} minutes to gain access.\nReply with the code you see in the image.",
            config.verification.timeout_minutes
        ))
        .add_file(attachment);

    let dm = member.user.direct_message(&ctx.http, message).await;

    if let Err(e) = dm {
        error!("Failed to send CAPTCHA DM to {}: {}", member.user.id, e);
    } else {
        info!("Sent CAPTCHA to user {} in guild {}", member.user.id, member.guild_id);
    }

    Ok(())
}

pub async fn verify_captcha_response(
    ctx: &Context,
    guild_id: GuildId,
    user_id: UserId,
    response: &str,
    config: &GuildConfig,
    data: Arc<BotData>,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(mut attempt) = data.database.queries.get_verification_attempt(&guild_id.to_string(), &user_id.to_string()).await? {
        if let Some(code) = &attempt.captcha_code {
            if code.eq_ignore_ascii_case(response) {
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

                data.database.queries.delete_verification_attempt(&guild_id.to_string(), &user_id.to_string()).await?;

                info!("User {} successfully verified in guild {}", user_id, guild_id);
                return Ok(true);
            } else {
                attempt.attempts += 1;
                
                if attempt.attempts >= 3 {
                    guild_id.kick_with_reason(&ctx.http, user_id, "Failed CAPTCHA verification").await?;
                    data.database.queries.delete_verification_attempt(&guild_id.to_string(), &user_id.to_string()).await?;
                    return Ok(false);
                }

                data.database.queries.update_verification_attempt(&attempt).await?;
            }
        }
    }

    Ok(false)
}
