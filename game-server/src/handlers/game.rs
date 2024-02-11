use early_returns::some_or_return;
use socketioxide::extract::{SocketRef, State};
use tokio_util::task::LocalPoolHandle;

use crate::state::{game_state::GameState, user_state::UserState};


pub fn activate(s: SocketRef, State(game_state): State<GameState>, State(pool): State<LocalPoolHandle>) {
    pool.spawn_pinned(move || async move {
        let game_store = game_state.games();
        let user = s.extensions.get::<UserState>().unwrap();
        let engine = some_or_return!(game_store.get(user.game_id));

        let script = format!(r#"
        game:player({}):select_slot({{"yokai-1", "yokai-3"}}, function(slot) print("selected slot: " .. slot) end)
        "#, engine.game.player_from_uuid(user.user_id).id);

        engine.load_script("debug", script.as_str()).await.ok();
    });
}
