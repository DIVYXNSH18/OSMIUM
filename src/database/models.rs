use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    pub guild_id: String,
    pub log_channel_id: Option<String>,
    pub antinuke: AntiNukeConfig,
    pub antiraid: AntiRaidConfig,
    pub antispam: AntiSpamConfig,
    pub antimention: AntiMentionConfig,
    pub antighost: AntiGhostConfig,
    pub antivanity: AntiVanityConfig,
    pub antirename: AntiRenameConfig,
    pub antiemoji: AntiEmojiConfig,
    pub antiinvite: AntiInviteConfig,
    pub verification: VerificationConfig,
    pub moderation: ModerationConfig,
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self {
            guild_id: String::new(),
            log_channel_id: None,
            antinuke: AntiNukeConfig::default(),
            antiraid: AntiRaidConfig::default(),
            antispam: AntiSpamConfig::default(),
            antimention: AntiMentionConfig::default(),
            antighost: AntiGhostConfig::default(),
            antivanity: AntiVanityConfig::default(),
            antirename: AntiRenameConfig::default(),
            antiemoji: AntiEmojiConfig::default(),
            antiinvite: AntiInviteConfig::default(),
            verification: VerificationConfig::default(),
            moderation: ModerationConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiNukeConfig {
    pub enabled: bool,
    pub ban_threshold: u32,
    pub kick_threshold: u32,
    pub channel_delete_threshold: u32,
    pub role_delete_threshold: u32,
    pub webhook_threshold: u32,
    pub punishment: PunishmentType,
    pub beast_mode_enabled: bool,
    pub whitelisted_users: Vec<String>,
    pub whitelisted_roles: Vec<String>,
}

impl Default for AntiNukeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ban_threshold: 3,
            kick_threshold: 3,
            channel_delete_threshold: 3,
            role_delete_threshold: 3,
            webhook_threshold: 5,
            punishment: PunishmentType::Ban,
            beast_mode_enabled: true,
            whitelisted_users: Vec::new(),
            whitelisted_roles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiRaidConfig {
    pub enabled: bool,
    pub score_limit: u32,
    pub score_reset_minutes: u32,
    pub join_row_threshold: u32,
    pub join_row_ms: u64,
    pub punishment: PunishmentType,
    pub lock_channels: bool,
}

impl Default for AntiRaidConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            score_limit: 50,
            score_reset_minutes: 5,
            join_row_threshold: 5,
            join_row_ms: 2000,
            punishment: PunishmentType::Kick,
            lock_channels: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiSpamConfig {
    pub enabled: bool,
    pub message_limit: u32,
    pub time_window_secs: u64,
    pub punishment: PunishmentType,
    pub duplicate_check: bool,
    pub emoji_limit: u32,
}

impl Default for AntiSpamConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            message_limit: 5,
            time_window_secs: 10,
            punishment: PunishmentType::Timeout,
            duplicate_check: true,
            emoji_limit: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiMentionConfig {
    pub enabled: bool,
    pub mention_limit: u32,
    pub punishment: PunishmentType,
    pub whitelisted_roles: Vec<String>,
}

impl Default for AntiMentionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mention_limit: 5,
            punishment: PunishmentType::Timeout,
            whitelisted_roles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiGhostConfig {
    pub enabled: bool,
    pub punishment: PunishmentType,
}

impl Default for AntiGhostConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            punishment: PunishmentType::Warn,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiVanityConfig {
    pub enabled: bool,
    pub punishment: PunishmentType,
}

impl Default for AntiVanityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            punishment: PunishmentType::Ban,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiRenameConfig {
    pub enabled: bool,
    pub server_rename: bool,
    pub server_icon: bool,
    pub role_rename: bool,
    pub channel_rename: bool,
    pub punishment: PunishmentType,
}

impl Default for AntiRenameConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            server_rename: true,
            server_icon: true,
            role_rename: true,
            channel_rename: true,
            punishment: PunishmentType::StripRoles,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiEmojiConfig {
    pub enabled: bool,
    pub punishment: PunishmentType,
}

impl Default for AntiEmojiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            punishment: PunishmentType::StripRoles,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiInviteConfig {
    pub enabled: bool,
    pub punishment: PunishmentType,
}

impl Default for AntiInviteConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            punishment: PunishmentType::Kick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub enabled: bool,
    pub method: VerificationMethod,
    pub unverified_role_id: Option<String>,
    pub verified_role_id: Option<String>,
    pub captcha_length: u8,
    pub captcha_difficulty: u8,
    pub timeout_minutes: u32,
    pub verification_channel_id: Option<String>,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: VerificationMethod::Button,
            unverified_role_id: None,
            verified_role_id: None,
            captcha_length: 6,
            captcha_difficulty: 3,
            timeout_minutes: 10,
            verification_channel_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationConfig {
    pub dm_on_action: bool,
    pub require_reason: bool,
}

impl Default for ModerationConfig {
    fn default() -> Self {
        Self {
            dm_on_action: true,
            require_reason: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PunishmentType {
    Ban,
    Kick,
    StripRoles,
    Timeout,
    Warn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum VerificationMethod {
    Captcha,
    Button,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationLog {
    pub guild_id: String,
    pub action: ModAction,
    pub moderator_id: String,
    pub target_id: String,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModAction {
    Ban,
    Kick,
    Mute,
    Unmute,
    Warn,
    Clear,
    Lock,
    Unlock,
    Slowmode,
    Nickname,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub guild_id: String,
    pub user_id: String,
    pub moderator_id: String,
    pub reason: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAttempt {
    pub guild_id: String,
    pub user_id: String,
    pub attempts: u32,
    pub captcha_code: Option<String>,
    pub timestamp: DateTime<Utc>,
}
