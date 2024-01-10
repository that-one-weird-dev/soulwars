use uuid::Uuid;

#[derive(Default, Clone)]
pub struct UserState {
    pub user_id: Uuid,
    pub game_id: Uuid,
}
