use std::sync::Arc;

use mlua::UserData;
use uuid::Uuid;

use crate::{
    card_storage::CardStorage, card_type::CardType, event_handler::EventHandler, player::Player,
};

pub struct Game {
    pub player1: Arc<Player>,
    pub player2: Arc<Player>,
}

impl Game {
    pub fn new(
        event_handler: Arc<Box<dyn EventHandler + Send + Sync>>,
        card_storage: Arc<CardStorage>,
        uuid_1: Uuid,
        uuid_2: Uuid,
        deck_1: Vec<CardType>,
        deck_2: Vec<CardType>,
    ) -> Self {
        let player_1 = Player::new(1, uuid_1, event_handler.clone(), card_storage.clone(), deck_1);
        let player_2 = Player::new(2, uuid_2, event_handler, card_storage, deck_2);

        Self {
            player1: Arc::new(player_1),
            player2: Arc::new(player_2),
        }
    }

    pub fn player_from_uuid(&self, uuid: Uuid) -> &Arc<Player> {
        if self.player1.uuid == uuid {
            &self.player1
        } else {
            &self.player2
        }
    }
}

impl UserData for Game {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method::<_, usize, _>("player", |_, this, player_id| match player_id {
            1 => Ok(this.player1.clone()),
            2 => Ok(this.player2.clone()),
            _ => Err(mlua::Error::runtime("Invalid player id")),
        });
    }
}
