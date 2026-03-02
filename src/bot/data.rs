use std::sync::Arc;
use dashmap::DashMap;
use serenity::model::id::{UserId, GuildId, MessageId};
use std::time::Instant;
use std::collections::VecDeque;
use crate::config::Config;
use crate::database::Database;

#[derive(Clone)]
pub struct BotData {
    pub config: Arc<Config>,
    pub database: Arc<Database>,
    pub redis_pool: Arc<deadpool_redis::Pool>,
    pub nuke_tracker: Arc<DashMap<(GuildId, UserId), Vec<Instant>>>,
    pub spam_tracker: Arc<DashMap<(GuildId, UserId), VecDeque<Instant>>>,
    pub message_cache: Arc<DashMap<MessageId, CachedMessage>>,
    pub beast_mode: Arc<DashMap<GuildId, BeastModeState>>,
    pub raid_joins: Arc<DashMap<GuildId, VecDeque<Instant>>>,
}

impl BotData {
    pub fn new(
        config: Arc<Config>,
        database: Arc<Database>,
        redis_pool: Arc<deadpool_redis::Pool>,
    ) -> Arc<Self> {
        Arc::new(Self {
            config,
            database,
            redis_pool,
            nuke_tracker: Arc::new(DashMap::new()),
            spam_tracker: Arc::new(DashMap::new()),
            message_cache: Arc::new(DashMap::new()),
            beast_mode: Arc::new(DashMap::new()),
            raid_joins: Arc::new(DashMap::new()),
        })
    }
}

#[derive(Clone)]
pub struct CachedMessage {
    pub content: String,
    pub author_id: UserId,
    pub channel_id: serenity::model::id::ChannelId,
    pub mentions: Vec<UserId>,
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct BeastModeState {
    pub activated_at: Instant,
    pub activated_by: String,
}

impl BeastModeState {
    pub fn is_active(&self) -> bool {
        self.activated_at.elapsed().as_secs() < 3600
    }
}
