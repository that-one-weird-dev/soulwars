use async_trait::async_trait;

use crate::{field_slot::FieldSlot, player::Player};

#[cfg(test)]
pub(crate) mod debug;

#[async_trait]
pub trait EventHandler {
    async fn select_slot(&self, player: &Player) -> anyhow::Result<FieldSlot>;
}
