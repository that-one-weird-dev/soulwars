use crate::{card_type::CardType, field_slot::FieldSlot};


#[derive(Default)]
pub struct Field {
    pub yokai_1: Option<CardType>,
    pub yokai_2: Option<CardType>,
    pub yokai_3: Option<CardType>,
    pub artifact_1: Option<CardType>,
    pub artifact_2: Option<CardType>,
    pub artifact_3: Option<CardType>,
    pub terrain: Option<CardType>,
    pub enchantment: Option<CardType>,
}

impl Field {
    pub fn get(&self, slot: FieldSlot) -> mlua::Result<&Option<CardType>> {
        match slot {
            FieldSlot::Yokai1 => Ok(&self.yokai_1),
            FieldSlot::Yokai2 => Ok(&self.yokai_2),
            FieldSlot::Yokai3 => Ok(&self.yokai_3),
            FieldSlot::Artifact1 => Ok(&self.artifact_1),
            FieldSlot::Artifact2 => Ok(&self.artifact_2),
            FieldSlot::Artifact3 => Ok(&self.artifact_3),
            FieldSlot::Terrain => Ok(&self.terrain),
            FieldSlot::Enchantment => Ok(&self.enchantment),
        }
    }

    pub fn set(&mut self, slot: FieldSlot, card: Option<CardType>) -> mlua::Result<()> {
        match slot {
            FieldSlot::Yokai1 => self.yokai_1 = card,
            FieldSlot::Yokai2 => self.yokai_2 = card,
            FieldSlot::Yokai3 => self.yokai_3 = card,
            FieldSlot::Artifact1 => self.artifact_1 = card,
            FieldSlot::Artifact2 => self.artifact_2 = card,
            FieldSlot::Artifact3 => self.artifact_3 = card,
            FieldSlot::Terrain => self.terrain = card,
            FieldSlot::Enchantment => self.terrain = card,
        };

        Ok(())
    }
}
