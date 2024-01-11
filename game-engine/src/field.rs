use crate::card_type::CardType;


#[derive(Default)]
pub struct Field {
    yokai_1: Option<CardType>,
    yokai_2: Option<CardType>,
    yokai_3: Option<CardType>,
    artifact_1: Option<CardType>,
    artifact_2: Option<CardType>,
    artifact_3: Option<CardType>,
    terrain: Option<CardType>,
    spell: Option<CardType>,
}

impl Field {
    pub fn get(&self, slot: String) -> mlua::Result<&Option<CardType>> {
        match slot.as_str() {
            "yokai-1" => Ok(&self.yokai_1),
            "yokai-2" => Ok(&self.yokai_2),
            "yokai-3" => Ok(&self.yokai_3),
            "artifact-1" => Ok(&self.artifact_1),
            "artifact-2" => Ok(&self.artifact_2),
            "artifact-3" => Ok(&self.artifact_3),
            "terrain" => Ok(&self.terrain),
            "spell" => Ok(&self.spell),
            _ => Err(mlua::Error::runtime("Invalid slot")),
        }
    }

    pub fn set(&mut self, slot: String, card: Option<CardType>) -> mlua::Result<()> {
        match slot.as_str() {
            "yokai-1" => self.yokai_1 = card,
            "yokai-2" => self.yokai_2 = card,
            "yokai-3" => self.yokai_3 = card,
            "artifact-1" => self.artifact_1 = card,
            "artifact-2" => self.artifact_2 = card,
            "artifact-3" => self.artifact_3 = card,
            "terrain" => self.terrain = card,
            "spell" => self.terrain = card,
            _ => return Err(mlua::Error::runtime("Invalid slot")),
        };

        Ok(())
    }
}
