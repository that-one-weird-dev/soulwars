use std::rc::Rc;

use mlua::{Table, TableExt, Result, Function};

use crate::{card::Card, player::Player};

pub struct CardScript<'a> {
    pub table: Table<'a>,
}

impl<'a> CardScript<'a> {
    pub(crate) fn new(table: Table<'a>) -> Self {
        Self { table }
    }

    pub fn activate(&self, card: Card, player: Rc<Player>) -> Result<()> {
        self.table.call_function::<_, ()>("activate", (card, player))
    }

    pub fn can_activate(&self, card: Card, player: Rc<Player>) -> Result<bool> {
        match self.table.get::<_, Function>("can_activate") {
            Ok(func) => {
                func.call((card, player))
            },
            _ => Ok(false),
        }
    }
}
