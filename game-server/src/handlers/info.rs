use std::sync::Arc;

use early_returns::some_or_return;
use game_engine::card_type::CardType;
use serde::Serialize;
use socketioxide::extract::{AckSender, SocketRef, State};
use tokio_util::task::LocalPoolHandle;
use tracing::info;

use crate::state::{game_state::GameState, user_state::UserState};

#[derive(Serialize)]
pub struct HandResponse {
    pub cards: Vec<CardType>,
}

pub async fn hand(s: SocketRef, ack: AckSender, State(game_state): State<GameState>, State(pool): State<LocalPoolHandle>) {
    pool.spawn_pinned(move || async move {
        let s = Arc::new(s);
        let game_store = game_state.games();

        info!("Requested hand from user");

        let user = s.extensions.get::<UserState>().unwrap();
        let engine = some_or_return!(game_store.get(user.game_id));

        let cards: Vec<u64> = engine
            .game
            .player1
            .deck
            .lock()
            .unwrap()
            .iter()
            .cloned()
            .take(5)
            .collect();

        engine.load_script("hand test", r#"
        game:player(1):select_slot(function(slot) print("selected slot: " .. slot) end)
        "#).await.ok();

        ack.send(HandResponse { cards }).ok();
    });
}
