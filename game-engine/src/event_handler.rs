use mlua::Result;

use crate::{field_slot::FieldSlot, player::Player};

pub type EventFn<Args, Ret> = Box<dyn Fn(&Player, Args) -> Result<Ret>>;

pub struct EventHandler {
    pub debug: EventFn<(), ()>,
    pub select_slot: EventFn<(), FieldSlot>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            debug: Box::new(|_, _| Ok(())),
            select_slot: Box::new(|_, _| Ok(FieldSlot::Yokai1)),
        }
    }
}
