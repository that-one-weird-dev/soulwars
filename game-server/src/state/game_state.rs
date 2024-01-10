use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use uuid::Uuid;

use crate::game::Game;

#[derive(Default)]
pub struct GameState {
    game_store: RwLock<GameStore>,
}

impl GameState {
    pub fn create_game(&self, id: Uuid) {
        let game = Game::new(id);
        self.games_mut().insert_game(game);
    }

    pub fn games(&self) -> std::sync::RwLockReadGuard<GameStore> {
        self.game_store.read().unwrap()
    }

    pub fn games_mut(&self) -> std::sync::RwLockWriteGuard<GameStore> {
        self.game_store.write().unwrap()
    }
}

#[derive(Default)]
pub struct GameStore {
    games: HashMap<Uuid, Mutex<Game>>,
}

impl GameStore {
    pub fn insert_game(&mut self, game: Game) {
        self.games.insert(game.id.clone(), Mutex::new(game));
    }

    pub fn get(&self, game_id: Uuid) -> Option<std::sync::MutexGuard<Game>> {
        self.games.get(&game_id).map(|game| game.lock().unwrap())
    }
}
