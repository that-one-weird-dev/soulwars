use std::cell::RefCell;

use mlua::{Function, UserData};

use crate::{card_type::CardType, field::Field};

pub struct Player {
    pub id: usize,
    pub hand: RefCell<Vec<CardType>>,
    pub deck: RefCell<Vec<CardType>>,
    pub field: RefCell<Field>,
}

impl Player {
    pub fn new(id: usize, deck: Vec<CardType>) -> Self {
        Self {
            id,
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

        methods.add_method::<_, (String, CardType), _>("summon", |_, this, (slot, card_type)| {
            let mut field = this.field.borrow_mut();

            field.set(slot, Some(card_type))?;

            Ok(card_type)
        });

        methods.add_method::<_, Function, _>("select_slot", |_, _, callback| {
            callback.call("yokai-2")?;

            Ok(())
        });
    }
}
