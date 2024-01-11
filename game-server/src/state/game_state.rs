use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use game_engine::engine::GameEngine;
use uuid::Uuid;

#[derive(Default)]
pub struct GameState {
    game_store: RwLock<GameStore>,
}

unsafe impl Send for GameState {}
unsafe impl Sync for GameState {}

impl GameState {
    pub fn create_game(&self, id: Uuid) {
        let game = GameEngine::new(id);
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
    games: HashMap<Uuid, Mutex<GameEngine>>,
}

impl GameStore {
    pub fn insert_game(&mut self, game: GameEngine) {
        self.games.insert(game.id.clone(), Mutex::new(game));
    }

    pub fn get(&self, game_id: Uuid) -> Option<std::sync::MutexGuard<GameEngine>> {
        self.games.get(&game_id).map(|game| game.lock().unwrap())
    }
}
