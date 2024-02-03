use async_trait::async_trait;

use crate::field_slot::FieldSlot;

use super::EventHandler;

pub struct DebugEventHandler {
    select_slot_value: FieldSlot,
}

impl DebugEventHandler {
    pub fn new() -> Self {
        Self {
            select_slot_value: FieldSlot::Yokai1,
        }
    }

    pub fn with_select_slot(mut self, value: FieldSlot) -> Self {
        self.select_slot_value = value;
        self
    }
}

#[async_trait]
impl EventHandler for DebugEventHandler {
    async fn select_slot(
        &self,
        _player: &crate::player::Player,
    ) -> anyhow::Result<FieldSlot> {
        Ok(self.select_slot_value.clone())
    }
}
