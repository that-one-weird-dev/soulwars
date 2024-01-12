use std::rc::Rc;

use mlua::UserData;

use crate::player::Player;

pub struct Game {
    pub player1: Rc<Player>,
    pub player2: Rc<Player>,
}

impl UserData for Game {
    fn add_methods<'lua, M: mlua::prelude::LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method::<_, usize, _>("player", |_, this, player_id| match player_id {
            1 => Ok(this.player1.clone()),
            2 => Ok(this.player2.clone()),
            _ => Err(mlua::Error::runtime("Invalid player id")),
        });
    }
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1: Rc::new(player1),
            player2: Rc::new(player2),
        }
    }
}
