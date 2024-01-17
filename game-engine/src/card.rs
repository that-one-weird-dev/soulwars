use std::rc::Rc;

use mlua::UserData;

use crate::card_data::CardData;

#[derive(Clone)]
pub struct Card {
    pub id: i64,
    pub data: Rc<CardData>,
}

impl Card {
    pub fn new(id: i64, data: CardData) -> Self {
        Self {
            id,
            data: Rc::new(data),
        }
    }
}

impl UserData for Card {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |_, this| Ok(this.id));
        fields.add_field_method_get("data", |_, this| Ok(this.data.clone()));
    }
}
