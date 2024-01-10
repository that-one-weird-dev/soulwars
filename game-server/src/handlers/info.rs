use early_returns::some_or_return;
use serde::Serialize;
use socketioxide::extract::{AckSender, SocketRef, State};
use tracing::info;

use crate::state::{user_state::UserState, game_state::GameState};

#[derive(Serialize)]
pub struct HandResponse {
    cards: Vec<i32>,
}

pub fn hand(s: SocketRef, ack: AckSender, State(game_state): State<GameState>) {
    let user = s.extensions.get::<UserState>().unwrap();
    let game_store = game_state.games();
    let game = some_or_return!(game_store.get(user.game_id));

    info!("Requested hand from user {}", user.user_id);

    ack.send(HandResponse { cards: vec![4, 2] }).ok();
}
