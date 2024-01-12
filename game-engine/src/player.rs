use std::{cell::RefCell, rc::Rc};

use mlua::{Function, UserData};

use crate::{
    card_type::CardType, event_handler::EventHandler, field::Field,
    field_slot::FieldSlot,
};

pub struct Player {
    pub id: usize,
    pub hand: RefCell<Vec<CardType>>,
    pub deck: RefCell<Vec<CardType>>,
    pub field: RefCell<Field>,

    pub event_handler: Rc<EventHandler>,
}

impl Player {
    pub fn new(id: usize, event_handler: Rc<EventHandler>, deck: Vec<CardType>) -> Self {
        Self {
            id,
            event_handler,
            hand: RefCell::new(Vec::new()),
            deck: RefCell::new(deck),
            field: RefCell::new(Field::default()),
        }
    }
}

impl UserData for Player {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id));
    }

    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("draw", |_, this, ()| {
            let card = this
                .deck
                .borrow_mut()
                .pop()
                .ok_or(mlua::Error::RuntimeError("No more cards".into()))?;
            this.hand.borrow_mut().push(card);

            Ok(())
        });

        methods.add_method::<_, (FieldSlot, CardType), _>(
            "summon",
            |_, this, (slot, card_type)| {
                let mut field = this.field.borrow_mut();

                field.set(slot, Some(card_type))?;

                Ok(card_type)
            },
        );

        methods.add_method::<_, Function, _>("select_slot", |_, this, callback| {
            let slot = (this.event_handler.select_slot)(&this, ());
            callback.call::<FieldSlot, _>(slot)?;

            Ok(())
        });
    }
}
