use std::{
    cell::RefCell,
    rc::Rc,
};

use mlua::{Function, UserData};

use crate::{
    card::Card, card_storage::CardStorage, card_type::CardType, event_handler::EventHandler,
    field::Field, field_slot::FieldSlot,
};

pub struct Player {
    pub id: usize,
    pub hand: RefCell<Vec<Card>>,
    pub deck: RefCell<Vec<CardType>>,
    pub field: RefCell<Field>,

    event_handler: Rc<EventHandler>,
    card_storage: Rc<CardStorage>,
}

impl Player {
    pub fn new(
        id: usize,
        event_handler: Rc<EventHandler>,
        card_storage: Rc<CardStorage>,
        deck: Vec<CardType>,
    ) -> Self {
        Self {
            id,
            event_handler,
            card_storage,
            hand: RefCell::new(Vec::new()),
            deck: RefCell::new(deck),
            field: RefCell::new(Field::default()),
        }
    }

    pub fn add_card_to_hand(&self, card: Card) {
        self.hand.borrow_mut().push(card);
    }

    pub fn draw(&self) -> mlua::Result<()> {
        let card_type = self
            .deck
            .borrow_mut()
            .pop()
            .ok_or(mlua::Error::RuntimeError("No more cards".into()))?;

        let card = self.card_storage.create(card_type)?;
        self.hand.borrow_mut().push(card);

        Ok(())
    }

    pub fn pop_from_hand(&self, index: usize) -> mlua::Result<Card> {
        let mut hand = self.hand.borrow_mut();
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
                let mut field = this.field.borrow_mut();

                field.set(slot, Some(card_type))?;

                Ok(card_type)
            },
        );

        methods.add_method("debug", |_, this, ()| {
            (this.event_handler.debug)(&this, ())?;

            Ok(())
        });
        methods.add_method::<_, Function, _>("select_slot", |_, this, callback| {
            let slot = (this.event_handler.select_slot)(&this, ())?;
            callback.call::<FieldSlot, _>(slot)?;

            Ok(())
        });
    }
}
