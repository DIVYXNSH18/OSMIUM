use serenity::{client::Context, model::prelude::*};
use crate::database::models::PunishmentType;

pub struct Logger<'a> {
    ctx: &'a Context,
    log_channel_id: ChannelId,
}

impl<'a> Logger<'a> {
    pub fn new(ctx: &'a Context, log_channel_id: &str) -> Self {
        let channel_id = log_channel_id.parse::<u64>().unwrap_or(0);
        Self {
            ctx,
            log_channel_id: ChannelId::new(channel_id),
        }
    }

    pub async fn log_member_join(&self, member: &Member) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("📥 Member Joined")
            .description(format!("<@{}> ({})", member.user.id, member.user.tag()))
            .field("Account Created", format!("<t:{}:R>", member.user.created_at().unix_timestamp()), false)
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_member_leave(&self, user: &User, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("📤 Member Left")
            .description(format!("<@{}> ({})", user.id, user.tag()))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_member_ban(&self, user: &User, guild_id: GuildId, reason: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🔨 Member Banned")
            .description(format!("<@{}> ({})", user.id, user.tag()))
            .field("Reason", reason.unwrap_or("No reason provided"), false)
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_member_unban(&self, user: &User, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✅ Member Unbanned")
            .description(format!("<@{}> ({})", user.id, user.tag()))
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_message_edit(&self, old: &Message, new: &Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✏️ Message Edited")
            .description(format!("By <@{}> in <#{}>", new.author.id, new.channel_id))
            .field("Before", &old.content, false)
            .field("After", &new.content, false)
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_channel_create(&self, channel: &GuildChannel) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("➕ Channel Created")
            .description(format!("<#{}> ({})", channel.id, channel.name))
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_channel_delete(&self, channel: &GuildChannel) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("➖ Channel Deleted")
            .description(format!("#{}", channel.name))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_role_create(&self, role: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("➕ Role Created")
            .description(format!("<@&{}> ({})", role.id, role.name))
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_role_delete(&self, role_id: RoleId, role_data: Option<Role>, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let name = role_data.map(|r| r.name).unwrap_or_else(|| "Unknown".to_string());
        let embed = CreateEmbed::new()
            .title("➖ Role Deleted")
            .description(format!("{} ({})", name, role_id))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_nuke_attempt(&self, action: &str, executor: UserId, punishment: &PunishmentType, count: u32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🚨 Anti-Nuke Triggered")
            .description(format!("**Action:** {}\n**Executor:** <@{}>\n**Count:** {}\n**Punishment:** {:?}", action, executor, count, punishment))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_raid_detected(&self, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🚨 Raid Detected")
            .description("Anti-raid measures have been activated. Channels locked and recent joiners punished.")
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_spam_detected(&self, user: &User, channel_id: ChannelId, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("⚠️ Spam Detected")
            .description(format!("**User:** <@{}>\n**Channel:** <#{}>\n**Punishment:** {:?}", user.id, channel_id, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_mention_abuse(&self, user: &User, channel_id: ChannelId, count: usize, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("⚠️ Mention Abuse")
            .description(format!("**User:** <@{}>\n**Channel:** <#{}>\n**Mentions:** {}\n**Punishment:** {:?}", user.id, channel_id, count, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_ghost_ping(&self, author: UserId, channel_id: ChannelId, mentions: &[UserId]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let mentions_str = mentions.iter().map(|id| format!("<@{}>", id)).collect::<Vec<_>>().join(", ");
        let embed = CreateEmbed::new()
            .title("👻 Ghost Ping Detected")
            .description(format!("**Author:** <@{}>\n**Channel:** <#{}>\n**Mentioned:** {}", author, channel_id, mentions_str))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_vanity_change(&self, executor: UserId, old: Option<&str>, new: Option<&str>, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🔗 Vanity URL Changed")
            .description(format!("**Executor:** <@{}>\n**Old:** {}\n**New:** {}\n**Punishment:** {:?}", 
                executor, 
                old.unwrap_or("None"), 
                new.unwrap_or("None"), 
                punishment))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_server_rename(&self, executor: UserId, old: &str, new: &str, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✏️ Server Renamed")
            .description(format!("**Executor:** <@{}>\n**Old:** {}\n**New:** {}\n**Punishment:** {:?}", executor, old, new, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_server_icon_change(&self, executor: UserId, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🖼️ Server Icon Changed")
            .description(format!("**Executor:** <@{}>\n**Punishment:** {:?}", executor, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_role_rename(&self, executor: UserId, old: &str, new: &str, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✏️ Role Renamed")
            .description(format!("**Executor:** <@{}>\n**Old:** {}\n**New:** {}\n**Punishment:** {:?}", executor, old, new, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_channel_rename(&self, executor: UserId, old: &str, new: &str, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✏️ Channel Renamed")
            .description(format!("**Executor:** <@{}>\n**Old:** {}\n**New:** {}\n**Punishment:** {:?}", executor, old, new, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_emoji_delete(&self, executor: UserId, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🗑️ Emoji Deleted")
            .description(format!("**Executor:** <@{}>\n**Punishment:** {:?}", executor, punishment))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_emoji_rename(&self, executor: UserId, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✏️ Emoji Renamed")
            .description(format!("**Executor:** <@{}>\n**Punishment:** {:?}", executor, punishment))
            .color(0xFFA500)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_invite_create(&self, event: &InviteCreateEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🔗 Invite Created")
            .description(format!("**Code:** {}\n**Channel:** <#{}>", event.code, event.channel_id))
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_invite_delete(&self, executor: UserId, code: &str, channel_id: ChannelId, punishment: &PunishmentType) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🗑️ Invite Deleted")
            .description(format!("**Executor:** <@{}>\n**Code:** {}\n**Channel:** <#{}>\n**Punishment:** {:?}", executor, code, channel_id, punishment))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_beast_mode_activated(&self, guild_id: GuildId, activated_by: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("🔥 Beast Mode Activated")
            .description(format!("**Activated By:** {}\n**Duration:** 1 hour\n\nAll suspicious actions will be immediately intervened.", activated_by))
            .color(0xFF0000)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }

    pub async fn log_beast_mode_deactivated(&self, guild_id: GuildId) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use serenity::builder::*;
        let embed = CreateEmbed::new()
            .title("✅ Beast Mode Deactivated")
            .description("Beast Mode has been automatically deactivated after 1 hour.")
            .color(0x00FF00)
            .timestamp(chrono::Utc::now());
        let message = CreateMessage::new().embed(embed);
        self.log_channel_id.send_message(&self.ctx.http, message).await?;
        Ok(())
    }
}
