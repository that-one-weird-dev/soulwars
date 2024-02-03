use std::sync::Arc;

use early_returns::ok_or_return;
use game_engine::{
    card::Card, card_data::CardData, card_storage::CardStorage, card_type::CardType,
    engine::GameEngine,
};
use serde::{Deserialize, Serialize};
use socketioxide::extract::{SocketRef, State, TryData};
use tracing::info;
use uuid::Uuid;

use crate::{
    engine::server_event_handler::ServerEventHandler,
    handlers::info::{self},
    state::{
        game_state::GameState,
        partial_game_state::{PartialGame, PartialGameState},
        user_state::UserState,
    },
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthMessage {
    user_id: String,
    game_id: String,
    deck: Vec<CardType>,
}

pub fn handle_connection(
    s: SocketRef,
    TryData(auth): TryData<AuthMessage>,
    State(game_state): State<GameState>,
    State(partial_game_state): State<PartialGameState>,
) {
    let auth = match auth {
        Ok(auth) => auth,
        Err(_) => {
            info!("invalid data format");
            s.disconnect().ok();
            return;
        }
    };

    let user_state = match parse_user_state(auth) {
        Some(state) => state,
        None => {
            info!("invalid data");
            s.disconnect().ok();
            return;
        }
    };

    info!("user connected with id {}", user_state.user_id);
    s.extensions.insert(user_state.clone());

    let s = Arc::new(s);

    let mut partial_games = partial_game_state.games.lock().unwrap();
    if let Some(game) = partial_games.get(&user_state.game_id) {
        let engine = create_game(
            game.user_socket.clone(),
            s.clone(),
            game.user_state.clone(),
            user_state.clone(),
        );
        game_state.create_game(user_state.game_id, ok_or_return!(engine));

        s.emit("game:ready", ()).ok();
        game.user_socket.emit("game:ready", ()).ok();
    } else {
        partial_games.insert(
            user_state.game_id,
            PartialGame {
                user_state: user_state.clone(),
                user_socket: s.clone(),
            },
        );
    }

    s.on("info:hand", info::hand);

    let game_id = user_state.game_id.clone();
    s.on_disconnect(
        move |s: SocketRef, State(partial_game_state): State<PartialGameState>| {
            let mut partial_games = partial_game_state.games.lock().unwrap();
            partial_games.remove(&game_id);

            s.broadcast().emit("user left", ()).ok();
        },
    );
}

fn parse_user_state(auth: AuthMessage) -> Option<UserState> {
    let user_id = Uuid::parse_str(&auth.user_id).ok()?;
    let game_id = Uuid::parse_str(&auth.game_id).ok()?;

    Some(UserState {
        user_id,
        game_id,
        deck: auth.deck,
    })
}

fn create_game(
    user1_socket: Arc<SocketRef>,
    user2_socket: Arc<SocketRef>,
    user1: UserState,
    user2: UserState,
) -> anyhow::Result<GameEngine> {
    let event_handler = ServerEventHandler {
        user1_socket,
        user2_socket,
    };

    let mut card_storage = CardStorage::new();

    for i in 1..=10 {
        card_storage.register(i, Card::new(i as i64, CardData::yokai(3, 3, 2)));
    }

    GameEngine::new(
        Box::new(event_handler),
        card_storage,
        user1.deck,
        user2.deck,
    )
}
