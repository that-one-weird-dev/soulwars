use std::rc::Rc;

use mlua::Lua;

use crate::{card::Card, card_wrapper::CardWrapper, game::Game};

pub struct GameEngine {
    pub game: Rc<Game>,

    lua: Lua,
}

impl GameEngine {
    pub fn new(game: Game) -> anyhow::Result<Self> {
        let lua = Lua::new();

        let game = Rc::new(game);

        lua.globals().set("game", game.clone())?;

        Ok(Self { game, lua })
    }

    pub fn load_card(&self, source: String, card: Card, id: i64) -> anyhow::Result<CardWrapper> {
        let table = self.lua.load(source).eval::<mlua::Table>()?;

        Ok(CardWrapper::create_card(table, card, id)?)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        card::{Card, EnchantmentKind},
        game::Game,
        player::Player,
    };

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
            Card::enchantment(EnchantmentKind::Normal),
            1,
        )?;

        card.activate(engine.game.player1.clone())?;

        assert!(matches!(engine.game.player1.hand.borrow().last(), Some(9)));

        Ok(())
    }

    #[test]
    fn should_summon_when_activated() -> anyhow::Result<()> {
        let game = Game::new(Player::new(1, vec![9]), Player::new(2, Vec::new()));

        let engine = GameEngine::new(game)?;
        let card = engine.load_card(
            r"
        return {
            activate = function(self, player)
                player:select_slot(function(slot)
                    player:summon(slot, self.id)
                end)
            end,
        }
        "
            .to_string(),
            Card::enchantment(EnchantmentKind::Normal),
            1,
        )?;

        card.activate(engine.game.player1.clone())?;

        assert!(matches!(
            engine.game.player1.field.borrow().yokai_1,
            Some(1)
        ));

        Ok(())
    }
}
