use std::sync::Arc;

use anyhow::anyhow;
use async_trait::async_trait;
use game_engine::{field_slot::FieldSlot, player::Player, event_handler::EventHandler};
use serde_json::Value;
use socketioxide::extract::SocketRef;
use tracing::info;

pub struct ServerEventHandler {
    pub user1_socket: Arc<SocketRef>,
    pub user2_socket: Arc<SocketRef>,
}

#[async_trait]
impl EventHandler for ServerEventHandler {
    async fn select_slot(&self, _player: &Player) -> anyhow::Result<FieldSlot> {
        info!("before request");
        let response = self.user1_socket.emit_with_ack::<Value>("game:select:field-slot", ());
        info!("1 is error: {}", matches!(response, Err(_)));
        let response = response?.await.map_err(|err| anyhow!("{}", err.to_string()));
        info!("2 is error: {}", matches!(response, Err(_)));

        info!("responded with {}", response?.data);

        // FieldSlot::try_from(response.as_str()).map_err(anyhow::Error::msg)
        Ok(FieldSlot::Yokai1)
    }
}
