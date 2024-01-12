use crate::{player::Player, field_slot::FieldSlot};

pub type EventFn<Args, Ret> = fn(player: &Player, args: Args) -> Ret;

pub struct EventHandler {
    pub select_slot: EventFn<(), FieldSlot>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            select_slot: |_, ()| FieldSlot::Yokai1,
        }
    }
}
