use game_engine::card_type::CardType;
use uuid::Uuid;

#[derive(Default, Clone)]
pub struct UserState {
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub deck: Vec<CardType>,
}
