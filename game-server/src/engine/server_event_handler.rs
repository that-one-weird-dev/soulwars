use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use game_engine::{field_slot::FieldSlot, player::Player, event_handler::EventHandler};
use socketioxide::extract::SocketRef;

pub struct ServerEventHandler {
    pub user1_socket: Arc<SocketRef>,
    pub user2_socket: Arc<SocketRef>,
}

#[async_trait]
impl EventHandler for ServerEventHandler {
    async fn select_slot(&self, _player: &Player) -> anyhow::Result<FieldSlot> {
        let response = self.user1_socket.emit_with_ack::<String>("game:select:field-slot", ());
        let response = response?.await.map_err(|err| anyhow!("{}", err.to_string()))?;

        FieldSlot::try_from(response.data.as_str()).map_err(anyhow::Error::msg)
    }
}
