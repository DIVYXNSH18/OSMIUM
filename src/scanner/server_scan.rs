use serenity::{client::Context, model::prelude::*};

pub struct ScanResult {
    pub findings: Vec<Finding>,
    pub score: u32,
}

pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub description: String,
}

pub enum Severity {
    Critical,
    Warning,
    Ok,
}

impl Severity {
    pub fn emoji(&self) -> &str {
        match self {
            Self::Critical => "🔴",
            Self::Warning => "🟡",
            Self::Ok => "🟢",
        }
    }
}

pub async fn scan_server(
    ctx: &Context,
    guild_id: GuildId,
) -> Result<ScanResult, Box<dyn std::error::Error + Send + Sync>> {
    let mut findings = Vec::new();
    let mut score = 100u32;

    let guild = guild_id.to_partial_guild(&ctx.http).await?;
    let roles = guild_id.roles(&ctx.http).await?;
    let channels = guild_id.channels(&ctx.http).await?;

    for (_, role) in &roles {
        if role.permissions.administrator() {
            findings.push(Finding {
                severity: Severity::Warning,
                title: format!("Administrator Role: {}", role.name),
                description: "This role has full administrator permissions. Ensure only trusted users have this role.".to_string(),
            });
            score = score.saturating_sub(5);
        }

        if role.permissions.manage_guild() || role.permissions.manage_channels() || role.permissions.manage_roles() {
            findings.push(Finding {
                severity: Severity::Warning,
                title: format!("Dangerous Permissions: {}", role.name),
                description: "This role has dangerous management permissions.".to_string(),
            });
            score = score.saturating_sub(3);
        }
    }

    if let Ok(members) = guild_id.members(&ctx.http, None, None).await {
        for member in members {
            if member.user.bot {
                let member_permissions = member.permissions(&ctx.cache)?;
                if member_permissions.administrator() {
                    findings.push(Finding {
                        severity: Severity::Critical,
                        title: format!("Bot with Admin: {}", member.user.name),
                        description: "This bot has administrator permissions. This is a security risk.".to_string(),
                    });
                    score = score.saturating_sub(10);
                }
            }
        }
    }

    for (_, channel) in &channels {
        if let Some(overwrites) = &channel.permission_overwrites {
            for overwrite in overwrites {
                if overwrite.allow.send_messages() && overwrite.kind == PermissionOverwriteType::Role(guild_id.everyone_role()) {
                    findings.push(Finding {
                        severity: Severity::Ok,
                        title: format!("Open Channel: {}", channel.name),
                        description: "@everyone can send messages in this channel.".to_string(),
                    });
                }
            }
        }
    }

    match guild.verification_level {
        VerificationLevel::None => {
            findings.push(Finding {
                severity: Severity::Critical,
                title: "No Verification Level".to_string(),
                description: "Server has no verification level. Enable at least Low verification.".to_string(),
            });
            score = score.saturating_sub(15);
        }
        VerificationLevel::Low => {
            findings.push(Finding {
                severity: Severity::Warning,
                title: "Low Verification Level".to_string(),
                description: "Consider increasing verification level for better security.".to_string(),
            });
            score = score.saturating_sub(5);
        }
        _ => {
            findings.push(Finding {
                severity: Severity::Ok,
                title: "Good Verification Level".to_string(),
                description: "Server has adequate verification level.".to_string(),
            });
        }
    }

    if guild.mfa_level == MfaLevel::None {
        findings.push(Finding {
            severity: Severity::Critical,
            title: "2FA Not Required".to_string(),
            description: "2FA is not required for moderators. Enable this in server settings.".to_string(),
        });
        score = score.saturating_sub(20);
    } else {
        findings.push(Finding {
            severity: Severity::Ok,
            title: "2FA Enabled".to_string(),
            description: "2FA is required for moderators.".to_string(),
        });
    }

    if let Ok(webhooks) = guild_id.webhooks(&ctx.http).await {
        if webhooks.len() > 10 {
            findings.push(Finding {
                severity: Severity::Warning,
                title: format!("Many Webhooks: {}", webhooks.len()),
                description: "Server has many webhooks. Review and remove unused ones.".to_string(),
            });
            score = score.saturating_sub(5);
        }
    }

    if let Ok(invites) = guild_id.invites(&ctx.http).await {
        let permanent_invites = invites.iter().filter(|i| i.max_age == 0).count();
        if permanent_invites > 0 {
            findings.push(Finding {
                severity: Severity::Warning,
                title: format!("Permanent Invites: {}", permanent_invites),
                description: "Some invites never expire. Consider setting expiration times.".to_string(),
            });
            score = score.saturating_sub(3);
        }
    }

    Ok(ScanResult { findings, score })
}
