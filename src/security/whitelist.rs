use serenity::model::prelude::*;
use crate::database::models::GuildConfig;

pub fn is_whitelisted(user_id: UserId, member: Option<&Member>, config: &GuildConfig) -> bool {
    if config.antinuke.whitelisted_users.contains(&user_id.to_string()) {
        return true;
    }

    if let Some(member) = member {
        for role_id in &member.roles {
            if config.antinuke.whitelisted_roles.contains(&role_id.to_string()) {
                return true;
            }
        }
    }

    false
}

pub fn is_mention_whitelisted(member: &Member, config: &GuildConfig) -> bool {
    for role_id in &member.roles {
        if config.antimention.whitelisted_roles.contains(&role_id.to_string()) {
            return true;
        }
    }
    false
}
