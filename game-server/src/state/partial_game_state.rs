use std::{sync::{Mutex, Arc}, collections::HashMap};

use socketioxide::extract::SocketRef;
use uuid::Uuid;

use super::user_state::UserState;

pub struct PartialGame {
    pub user_state: UserState,
    pub user_socket: Arc<SocketRef>,
}

#[derive(Default)]
pub struct PartialGameState {
    pub games: Mutex<HashMap<Uuid, PartialGame>>,
}

unsafe impl Send for PartialGameState {}
unsafe impl Sync for PartialGameState {}
