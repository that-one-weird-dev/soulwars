use std::sync::{Arc, Mutex};

use mlua::{Function, UserData};

use crate::{
    card::Card, card_storage::CardStorage, card_type::CardType, event_handler::EventHandler,
    field::Field, field_slot::FieldSlot,
};

pub struct Player {
    pub id: usize,
    pub hand: Mutex<Vec<Card>>,
    pub deck: Mutex<Vec<CardType>>,
    pub field: Mutex<Field>,

    event_handler: Arc<Box<dyn EventHandler + Send + Sync>>,
    card_storage: Arc<CardStorage>,
}

unsafe impl Sync for Player {}

impl Player {
    pub fn new(
        id: usize,
        event_handler: Arc<Box<dyn EventHandler + Send + Sync>>,
        card_storage: Arc<CardStorage>,
        deck: Vec<CardType>,
    ) -> Self {
        Self {
            id,
            event_handler,
            card_storage,
            hand: Mutex::new(Vec::new()),
            deck: Mutex::new(deck),
            field: Mutex::new(Field::default()),
        }
    }

    pub fn add_card_to_hand(&self, card: Card) {
        self.hand.lock().unwrap().push(card);
    }

    pub fn draw(&self) -> mlua::Result<()> {
        let card_type = self
            .deck
            .lock()
            .unwrap()
            .pop()
            .ok_or(mlua::Error::RuntimeError("No more cards".into()))?;

        let card = self.card_storage.create(card_type)?;
        self.hand.lock().unwrap().push(card);

        Ok(())
    }

    pub fn pop_from_hand(&self, index: usize) -> mlua::Result<Card> {
        let mut hand = self.hand.lock().unwrap();
        if hand.len() <= index {
            return Err(mlua::Error::runtime("Invalid hand index"));
        }

        Ok(hand.remove(index))
    }
}

impl UserData for Player {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id));
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("draw", |_, this, ()| this.draw());

        methods.add_method::<_, (FieldSlot, CardType), _>(
            "summon",
            |_, this, (slot, card_type)| {
                let mut field = this.field.lock().unwrap();

                field.set(slot, Some(card_type))?;

                Ok(card_type)
            },
        );

        methods.add_async_method("select_slot", |_, this, callback: Function| async move {
            let slot = this
                .event_handler
                .select_slot(&this)
                .await
                .map_err(|err| mlua::Error::external(err.to_string()))?;
            callback.call::<FieldSlot, _>(slot)?;

            Ok(())
        });
    }
}
