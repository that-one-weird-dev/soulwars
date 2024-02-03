use std::sync::Arc;

use async_trait::async_trait;
use game_engine::{field_slot::FieldSlot, player::Player, event_handler::EventHandler};
use socketioxide::extract::SocketRef;

pub struct ServerEventHandler {
    pub user1_socket: Arc<SocketRef>,
    pub user2_socket: Arc<SocketRef>,
}

#[async_trait]
impl EventHandler for ServerEventHandler {
    fn select_slot(&self, _player: &Player) -> anyhow::Result<FieldSlot>  {
        Ok(FieldSlot::Yokai3)
    }
}
