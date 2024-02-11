use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use game_engine::{field_slot::FieldSlot, player::Player, event_handler::EventHandler};
use socketioxide::extract::SocketRef;

pub struct ServerEventHandler {
    pub user1_socket: Arc<SocketRef>,
    pub user2_socket: Arc<SocketRef>,
}

impl ServerEventHandler {
    fn player_socket(&self, player: &Player) -> &Arc<SocketRef> {
        match player.id {
            1 => &self.user1_socket,
            _ => &self.user2_socket,
        }
    }
}

#[async_trait]
impl EventHandler for ServerEventHandler {
    async fn select_slot(&self, player: &Player, slots: Vec<FieldSlot>) -> anyhow::Result<FieldSlot> {
        let response = self.player_socket(player)
            .timeout(Duration::from_secs(30))
            .emit_with_ack::<Vec<String>>("game:select:field-slot", (slots,))?
            .await;

        match response {
            Ok(response) => match &response.data[..] {
                [data] => FieldSlot::try_from(data.as_str()).map_err(anyhow::Error::msg),
                _ => Ok(FieldSlot::Yokai1),
            },
            Err(_) => Ok(FieldSlot::Yokai1),
        }
    }
}
