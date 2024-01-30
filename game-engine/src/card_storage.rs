use std::collections::HashMap;

use crate::{card::Card, card_type::CardType};

#[derive(Default)]
pub struct CardStorage {
    storage: HashMap<CardType, Card>,
}

impl CardStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: CardType, card: Card) {
        self.storage.insert(id, card);
    }

    pub fn create(&self, id: CardType) -> mlua::Result<Card> {
        self.storage.get(&id).cloned().ok_or(mlua::Error::runtime(format!("No card with id {id}")))
    }
}
