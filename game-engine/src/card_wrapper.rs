use std::sync::Arc;

use crate::{card::Card, player::Player};

pub struct CardWrapper<'a> {
    table: mlua::Table<'a>,
}

impl<'a> CardWrapper<'a> {
    pub fn create_card(
        card_table: mlua::Table<'a>,
        card: Card,
        id: i64,
    ) -> mlua::Result<Self> {
        card_table.set("data", card)?;
        card_table.set("id", id)?;

        Ok(Self { table: card_table })
    }

    pub fn activate(&self, player: Arc<Player>) -> anyhow::Result<()> {
        let activate = self.table.get::<_, mlua::Function>("activate")?;

        Ok(activate.call((self.table.clone(), player))?)
    }
}
