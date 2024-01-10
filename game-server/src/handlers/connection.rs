use serde::{Deserialize, Serialize};
use socketioxide::extract::{SocketRef, TryData};
use tracing::info;
use uuid::Uuid;

use crate::state::user_state::UserState;

use super::info;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthMessage {
    user_id: String,
    game_id: String,
}

pub fn handle_connection(s: SocketRef, TryData(auth): TryData<AuthMessage>) {
    let auth = match auth {
        Ok(auth) => auth,
        Err(_) => {
            s.disconnect().ok();
            return;
        }
    };

    let user_state = match parse_user_state(auth) {
        Some(state) => state,
        None => {
            s.disconnect().ok();
            return
        },
    };

    info!("user connected with id {}", user_state.user_id);

    s.extensions.insert(user_state);


    s.on("info:hand", info::hand);

    s.on_disconnect(|s: SocketRef| {
        s.broadcast().emit("user left", ()).ok();
    });
}

fn parse_user_state(auth: AuthMessage) -> Option<UserState> {
    let user_id = Uuid::parse_str(&auth.user_id).ok()?;
    let game_id = Uuid::parse_str(&auth.game_id).ok()?;

    Some(UserState { user_id, game_id })
}
