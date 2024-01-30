use std::rc::Rc;

use mlua::{AsChunk, Lua, Table};

use crate::{
    card_script::CardScript, card_storage::CardStorage, card_type::CardType,
    event_handler::EventHandler, game::Game, player::Player,
};

pub enum PlayerId {
    Player1,
    Player2,
}

pub struct GameEngine {
    pub game: Rc<Game>,
    pub event_handler: Rc<EventHandler>,
    pub card_storage: Rc<CardStorage>,

    lua: Lua,
}

impl GameEngine {
    pub fn new(
        event_handler: EventHandler,
        card_storage: CardStorage,
        deck_1: Vec<CardType>,
        deck_2: Vec<CardType>,
    ) -> anyhow::Result<Self> {
        let event_handler = Rc::new(event_handler);
        let card_storage = Rc::new(card_storage);

        let game = Game::new(event_handler.clone(), card_storage.clone(), deck_1, deck_2);
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

        let engine = Self {
            game,
            event_handler,
            card_storage,
            lua,
        };

        Ok(engine)
    }

    pub fn load_script<'a>(
        &'a self,
        name: impl Into<String>,
        source: impl AsChunk<'a, 'a>,
    ) -> anyhow::Result<()> {
        Ok(self.lua.load(source).set_name(name).exec()?)
    }

    pub fn get_card_script(&self, id: i64) -> anyhow::Result<CardScript> {
        let cards = self.lua.globals().get::<_, Table>("__cards")?;
        let card_table = cards.get::<_, Table>(id)?;

        Ok(CardScript::new(card_table))
    }

    pub fn draw(&self, player_id: PlayerId) -> anyhow::Result<()> {
        self.get_player(player_id).draw()?;

        Ok(())
    }

    pub fn activate_card_from_hand(&self, player_id: PlayerId, index: usize) -> anyhow::Result<()> {
        let player = self.get_player(player_id);

        let card = player.pop_from_hand(index)?;
        let card_script = self.get_card_script(card.id)?;

        card_script.activate(card, player.clone())?;

        Ok(())
    }

    fn get_player(&self, player_id: PlayerId) -> &Rc<Player> {
        match player_id {
            PlayerId::Player1 => &self.game.player1,
            PlayerId::Player2 => &self.game.player2,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        card::Card,
        card_data::{CardData, EnchantmentKind},
        card_storage::CardStorage,
        event_handler::EventHandler,
        field_slot::FieldSlot, engine::PlayerId,
    };

    use super::GameEngine;

    #[test]
    fn should_summon_when_activated() -> anyhow::Result<()> {
        let event_handler = EventHandler {
            select_slot: Box::new(|_, ()| Ok(FieldSlot::Yokai2)),
            ..Default::default()
        };

        let mut card_storage = CardStorage::new();
        card_storage.register(
            1,
            Card::new(1, CardData::enchantment(EnchantmentKind::Normal)),
        );

        let engine = GameEngine::new(event_handler, card_storage, vec![9], Vec::new())?;
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

        let card_script = engine.get_card_script(1)?;
        let card = engine.card_storage.create(1)?;

        card_script.activate(card, engine.game.player1.clone())?;

        assert!(matches!(
            engine.game.player1.field.borrow().yokai_2,
            Some(1)
        ));

        Ok(())
    }

    #[test]
    fn should_draw_and_activate() -> anyhow::Result<()> {
        let event_handler = EventHandler {
            select_slot: Box::new(|_, ()| Ok(FieldSlot::Yokai3)),
            ..Default::default()
        };

        let mut card_storage = CardStorage::new();
        card_storage.register(
            1,
            Card::new(1, CardData::enchantment(EnchantmentKind::Normal)),
        );

        let engine = GameEngine::new(event_handler, card_storage, vec![1], Vec::new())?;
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

        engine.draw(PlayerId::Player1)?;
        engine.activate_card_from_hand(PlayerId::Player1, 0)?;

        assert!(matches!(
            engine.game.player1.field.borrow().yokai_3,
            Some(1)
        ));

        Ok(())
    }
}
