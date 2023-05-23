use crate::interfaces::Activity;
use crate::utils::Snowflake;
use crate::{entities::User, interfaces::ClientStatusObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
/// See https://discord.com/developers/docs/topics/gateway-events#presence-update-presence-update-event-fields
pub struct PresenceUpdate {
    pub user: User,
    pub guild_id: Snowflake,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatusObject,
}
