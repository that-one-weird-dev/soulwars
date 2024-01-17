use std::rc::Rc;

use mlua::{AsChunk, Lua, Table};

use crate::{
    card_script::CardScript, card_type::CardType, event_handler::EventHandler, game::Game,
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

        lua.load(
            r#"
        __cards = {}

        function create_card(id, card)
            __cards[id] = card
        end

        function get_card(id)
            return __cards[id]
        end
        "#,
        )
        .set_name("engine")
        .exec()?;

        let engine = Self { game, lua };

        Ok(engine)
    }

    pub fn load_script<'a>(
        &'a self,
        name: impl Into<String>,
        source: impl AsChunk<'a, 'a>,
    ) -> mlua::Result<()> {
        self.lua.load(source).set_name(name).exec()
    }

    pub fn get_card_script(&self, id: i64) -> mlua::Result<CardScript> {
        let cards = self.lua.globals().get::<_, Table>("__cards")?;
        let card_table = cards.get::<_, Table>(id)?;

        Ok(CardScript::new(card_table))
    }
}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        card::Card,
        card_data::{CardData, EnchantmentKind},
        event_handler::EventHandler,
        field_slot::FieldSlot,
    };

    use super::GameEngine;

    #[test]
    fn should_summon_when_activated() -> anyhow::Result<()> {
        let event_handler = EventHandler {
            select_slot: Box::new(|_, ()| Ok(FieldSlot::Yokai2)),
            ..Default::default()
        };

        let engine = GameEngine::new(event_handler, vec![9], Vec::new())?;
        engine.load_script(
            "test",
            r"
        create_card(1, {
            activate = function(self, player)
                player:select_slot(function(slot)
                    player:summon(slot, self.id)
                end)
            end,
        })
        ",
        )?;

        let card = Card::new(1, CardData::enchantment(EnchantmentKind::Normal));

        let card_script = engine.get_card_script(1)?;
        card_script.activate(card, engine.game.player1.clone())?;

        assert!(matches!(
            engine.game.player1.field.borrow().yokai_2,
            Some(1)
        ));

        Ok(())
    }
}
