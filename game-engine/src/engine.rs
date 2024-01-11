use std::sync::Arc;

use mlua::Lua;

use crate::{card::Card, card_wrapper::CardWrapper, game::Game};

pub struct GameEngine {
    pub game: Arc<Game>,

    lua: Lua,
}

impl GameEngine {
    pub fn new(game: Game) -> anyhow::Result<Self> {
        let lua = Lua::new();

        let game = Arc::new(game);

        lua.globals().set("game", game.clone())?;

        Ok(Self { game, lua })
    }

    pub fn load_card(&self, source: String) -> anyhow::Result<CardWrapper> {
        let table = self.lua.load(source).eval::<mlua::Table>()?;

        Ok(CardWrapper::create_card(
            &self.lua,
            table,
            Card::Artifact {},
        )?)
    }
}

#[cfg(test)]
mod test {
    use crate::{game::Game, player::Player};

    use super::GameEngine;

    #[test]
    fn should_draw_when_activated() -> anyhow::Result<()> {
        let game = Game::new(Player::new(1, vec![9]), Player::new(2, Vec::new()));

        let engine = GameEngine::new(game)?;
        let card = engine.load_card(
            r"
        return {
            activate = function(self, player)
                player:draw()
            end,
        }
        "
            .to_string(),
        )?;

        card.activate(engine.game.player1.clone())?;

        assert!(matches!(engine.game.player1.hand.borrow().last(), Some(9)));

        Ok(())
    }
}
