use std::rc::Rc;

use mlua::Lua;

use crate::{
    card::Card, card_type::CardType, card_wrapper::CardWrapper, event_handler::EventHandler,
    game::Game,
};

pub struct GameEngine {
    pub game: Rc<Game>,

    lua: Lua,
}

impl GameEngine {
    pub fn new(
        event_handler: EventHandler,
        deck_1: Vec<CardType>,
        deck_2: Vec<CardType>,
    ) -> anyhow::Result<Self> {
        let event_handler = Rc::new(event_handler);
        let game = Game::new(event_handler.clone(), deck_1, deck_2);
        let game = Rc::new(game);

        let lua = Lua::new();
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
        event_handler::EventHandler,
        field_slot::FieldSlot,
    };

    use super::GameEngine;

    #[test]
    fn should_draw_when_activated() -> anyhow::Result<()> {
        let engine = GameEngine::new(EventHandler::default(), vec![9], Vec::new())?;
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
        let event_handler = EventHandler {
            select_slot: |_, ()| FieldSlot::Yokai2,
            ..Default::default()
        };

        let engine = GameEngine::new(event_handler, vec![9], Vec::new())?;
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
            engine.game.player1.field.borrow().yokai_2,
            Some(1)
        ));

        Ok(())
    }
}
