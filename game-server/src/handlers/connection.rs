use serde::{Deserialize, Serialize};
use socketioxide::extract::{SocketRef, State, TryData};
use tracing::info;
use uuid::Uuid;

use crate::user_state::UserState;

use super::info;

#[derive(Serialize, Deserialize)]
pub struct AuthMessage {
    id: String,
}

pub fn handle_connection(s: SocketRef, TryData(auth): TryData<AuthMessage>, state: State<UserState>) {
    let auth = match auth {
        Ok(auth) => auth,
        Err(_) => {
            s.disconnect().ok();
            return
        },
    };

    let user_id = match Uuid::parse_str(auth.id.as_str()) {
        Ok(id) => id,
        _ => return,
    };
    state.init(user_id);

    info!("user connected with id {}", auth.id);

    s.on("info:hand", info::hand);

    s.on_disconnect(|s: SocketRef| {
        s.broadcast().emit("user left", ()).ok();
    });
}
