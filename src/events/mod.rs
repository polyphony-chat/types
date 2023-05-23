use serde::{Deserialize, Serialize};

mod identify;
mod presence;
mod resume;
mod ready;
mod hello;
mod request_members;
mod voice_status;
mod heartbeat;
mod guild;
mod channel;
mod user;
mod thread;

pub use identify::*;
pub use presence::*;
pub use resume::*;
pub use ready::*;
pub use hello::*;
pub use request_members::*;
pub use voice_status::*;
pub use heartbeat::*;
pub use guild::*;
pub use channel::*;
pub use user::*;
pub use thread::*;

pub trait WebSocketEvent {}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Option<serde_json::Value>,
    pub s: Option<u64>,
    pub t: Option<String>,
}

impl WebSocketEvent for GatewayPayload {}