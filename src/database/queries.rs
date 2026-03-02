use mongodb::{bson::doc, Collection};
use crate::database::models::*;
use chrono::Utc;

pub struct Queries {
    guild_configs: Collection<GuildConfig>,
    moderation_logs: Collection<ModerationLog>,
    warnings: Collection<Warning>,
    verification_attempts: Collection<VerificationAttempt>,
}

impl Queries {
    pub fn new(db: &mongodb::Database) -> Self {
        Self {
            guild_configs: db.collection("guild_configs"),
            moderation_logs: db.collection("moderation_logs"),
            warnings: db.collection("warnings"),
            verification_attempts: db.collection("verification_attempts"),
        }
    }

    pub async fn get_guild_config(&self, guild_id: &str) -> Result<GuildConfig, mongodb::error::Error> {
        match self.guild_configs.find_one(doc! { "guild_id": guild_id }, None).await? {
            Some(config) => Ok(config),
            None => {
                let config = GuildConfig {
                    guild_id: guild_id.to_string(),
                    ..Default::default()
                };
                self.guild_configs.insert_one(&config, None).await?;
                Ok(config)
            }
        }
    }

    pub async fn update_guild_config(&self, config: &GuildConfig) -> Result<(), mongodb::error::Error> {
        self.guild_configs
            .replace_one(doc! { "guild_id": &config.guild_id }, config, None)
            .await?;
        Ok(())
    }

    pub async fn add_moderation_log(&self, log: ModerationLog) -> Result<(), mongodb::error::Error> {
        self.moderation_logs.insert_one(log, None).await?;
        Ok(())
    }

    pub async fn get_moderation_logs(&self, guild_id: &str, limit: i64) -> Result<Vec<ModerationLog>, mongodb::error::Error> {
        use futures::stream::TryStreamExt;
        let mut cursor = self.moderation_logs
            .find(doc! { "guild_id": guild_id }, None)
            .await?;
        
        let mut logs = Vec::new();
        while let Some(log) = cursor.try_next().await? {
            logs.push(log);
            if logs.len() >= limit as usize {
                break;
            }
        }
        Ok(logs)
    }

    pub async fn add_warning(&self, warning: Warning) -> Result<(), mongodb::error::Error> {
        self.warnings.insert_one(warning, None).await?;
        Ok(())
    }

    pub async fn get_warnings(&self, guild_id: &str, user_id: &str) -> Result<Vec<Warning>, mongodb::error::Error> {
        use futures::stream::TryStreamExt;
        let mut cursor = self.warnings
            .find(doc! { "guild_id": guild_id, "user_id": user_id }, None)
            .await?;
        
        let mut warnings = Vec::new();
        while let Some(warning) = cursor.try_next().await? {
            warnings.push(warning);
        }
        Ok(warnings)
    }

    pub async fn clear_warnings(&self, guild_id: &str, user_id: &str) -> Result<(), mongodb::error::Error> {
        self.warnings
            .delete_many(doc! { "guild_id": guild_id, "user_id": user_id }, None)
            .await?;
        Ok(())
    }

    pub async fn get_verification_attempt(&self, guild_id: &str, user_id: &str) -> Result<Option<VerificationAttempt>, mongodb::error::Error> {
        self.verification_attempts
            .find_one(doc! { "guild_id": guild_id, "user_id": user_id }, None)
            .await
    }

    pub async fn update_verification_attempt(&self, attempt: &VerificationAttempt) -> Result<(), mongodb::error::Error> {
        self.verification_attempts
            .replace_one(
                doc! { "guild_id": &attempt.guild_id, "user_id": &attempt.user_id },
                attempt,
                None,
            )
            .await?;
        Ok(())
    }

    pub async fn create_verification_attempt(&self, guild_id: &str, user_id: &str, captcha_code: Option<String>) -> Result<(), mongodb::error::Error> {
        let attempt = VerificationAttempt {
            guild_id: guild_id.to_string(),
            user_id: user_id.to_string(),
            attempts: 0,
            captcha_code,
            timestamp: Utc::now(),
        };
        self.verification_attempts.insert_one(attempt, None).await?;
        Ok(())
    }

    pub async fn delete_verification_attempt(&self, guild_id: &str, user_id: &str) -> Result<(), mongodb::error::Error> {
        self.verification_attempts
            .delete_one(doc! { "guild_id": guild_id, "user_id": user_id }, None)
            .await?;
        Ok(())
    }
}
