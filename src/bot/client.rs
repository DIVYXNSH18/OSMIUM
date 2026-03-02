use std::sync::Arc;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    model::prelude::*,
    prelude::*,
};
use poise::serenity_prelude as serenity_prelude;
use tracing::{info, error};
use crate::bot::data::BotData;
use crate::events;
use crate::commands;

pub struct Handler {
    pub data: Arc<BotData>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        if let Err(e) = events::guild_member_add::handle(&ctx, &new_member, self.data.clone()).await {
            error!("Error in guild_member_addition: {}", e);
        }
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member: Option<Member>) {
        if let Err(e) = events::guild_member_remove::handle(&ctx, guild_id, &user, self.data.clone()).await {
            error!("Error in guild_member_removal: {}", e);
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = events::message_create::handle(&ctx, &msg, self.data.clone()).await {
            error!("Error in message: {}", e);
        }
    }

    async fn message_delete(&self, ctx: Context, channel_id: ChannelId, deleted_message_id: MessageId, guild_id: Option<GuildId>) {
        if let Err(e) = events::message_delete::handle(&ctx, channel_id, deleted_message_id, guild_id, self.data.clone()).await {
            error!("Error in message_delete: {}", e);
        }
    }

    async fn message_update(&self, ctx: Context, old: Option<Message>, new: Option<Message>, event: MessageUpdateEvent) {
        if let Err(e) = events::message_update::handle(&ctx, old, new, event, self.data.clone()).await {
            error!("Error in message_update: {}", e);
        }
    }

    async fn guild_ban_addition(&self, ctx: Context, guild_id: GuildId, banned_user: User) {
        if let Err(e) = events::guild_ban_add::handle(&ctx, guild_id, &banned_user, self.data.clone()).await {
            error!("Error in guild_ban_addition: {}", e);
        }
    }

    async fn guild_ban_removal(&self, ctx: Context, guild_id: GuildId, unbanned_user: User) {
        if let Err(e) = events::guild_ban_remove::handle(&ctx, guild_id, &unbanned_user, self.data.clone()).await {
            error!("Error in guild_ban_removal: {}", e);
        }
    }

    async fn channel_create(&self, ctx: Context, channel: &GuildChannel) {
        if let Err(e) = events::channel_create::handle(&ctx, channel, self.data.clone()).await {
            error!("Error in channel_create: {}", e);
        }
    }

    async fn channel_delete(&self, ctx: Context, channel: &GuildChannel) {
        if let Err(e) = events::channel_delete::handle(&ctx, channel, self.data.clone()).await {
            error!("Error in channel_delete: {}", e);
        }
    }

    async fn channel_update(&self, ctx: Context, old: Option<Channel>, new: Channel) {
        if let Err(e) = events::channel_update::handle(&ctx, old, new, self.data.clone()).await {
            error!("Error in channel_update: {}", e);
        }
    }

    async fn guild_role_create(&self, ctx: Context, new: Role) {
        if let Err(e) = events::role_create::handle(&ctx, &new, self.data.clone()).await {
            error!("Error in guild_role_create: {}", e);
        }
    }

    async fn guild_role_delete(&self, ctx: Context, guild_id: GuildId, removed_role_id: RoleId, removed_role_data_if_available: Option<Role>) {
        if let Err(e) = events::role_delete::handle(&ctx, guild_id, removed_role_id, removed_role_data_if_available, self.data.clone()).await {
            error!("Error in guild_role_delete: {}", e);
        }
    }

    async fn guild_role_update(&self, ctx: Context, old: Option<Role>, new: Role) {
        if let Err(e) = events::role_update::handle(&ctx, old, new, self.data.clone()).await {
            error!("Error in guild_role_update: {}", e);
        }
    }

    async fn guild_update(&self, ctx: Context, old: Option<Guild>, new: PartialGuild) {
        if let Err(e) = events::guild_update::handle(&ctx, old, new, self.data.clone()).await {
            error!("Error in guild_update: {}", e);
        }
    }

    async fn invite_create(&self, ctx: Context, data: InviteCreateEvent) {
        if let Err(e) = events::invite_create::handle(&ctx, data, self.data.clone()).await {
            error!("Error in invite_create: {}", e);
        }
    }

    async fn invite_delete(&self, ctx: Context, data: InviteDeleteEvent) {
        if let Err(e) = events::invite_delete::handle(&ctx, data, self.data.clone()).await {
            error!("Error in invite_delete: {}", e);
        }
    }

    async fn guild_emojis_update(&self, ctx: Context, guild_id: GuildId, current_state: std::collections::HashMap<EmojiId, Emoji>) {
        if let Err(e) = events::emoji_update::handle(&ctx, guild_id, current_state, self.data.clone()).await {
            error!("Error in guild_emojis_update: {}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(e) = events::interaction_create::handle(&ctx, interaction, self.data.clone()).await {
            error!("Error in interaction_create: {}", e);
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Bot is ready as {}", ready.user.name);
    }
}

pub async fn create_client(data: Arc<BotData>) -> Result<Client, serenity::Error> {
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS
        | GatewayIntents::GUILD_INTEGRATIONS
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::moderation::ban(),
                commands::moderation::kick(),
                commands::moderation::mute(),
                commands::moderation::unmute(),
                commands::moderation::warn(),
                commands::moderation::warnings(),
                commands::moderation::clearwarnings(),
                commands::moderation::clear(),
                commands::moderation::lock(),
                commands::moderation::unlock(),
                commands::moderation::lockall(),
                commands::moderation::unlockall(),
                commands::moderation::slowmode(),
                commands::moderation::nickname(),
                commands::scan::scan(),
                commands::security::security(),
                commands::whitelist::whitelist(),
                commands::verification::setup_verification(),
                commands::verification::send_verify_button(),
                commands::config::setlogchannel(),
                commands::config::config(),
            ],
            ..Default::default()
        })
        .token(&data.config.discord_token)
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data.clone())
            })
        })
        .build()
        .await?;

    Ok(framework.client())
}
