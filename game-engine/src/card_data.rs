use mlua::{IntoLua, UserData};

macro_rules! add_card_field {
    ($fields:expr, $variant:ident, $field:ident, $value:expr) => {
        $fields.add_field_method_get("$field", |lua, this| match this {
            CardData::$variant { $field, .. } => IntoLua::into_lua($value, lua),
            _ => Ok(mlua::Value::Nil),
        });
    };
}

pub enum EnchantmentKind {
    Normal,
    Blessing,
    Curse,
}

impl EnchantmentKind {
    fn as_str(&self) -> &'static str {
        match self {
            EnchantmentKind::Normal => "normal",
            EnchantmentKind::Blessing => "blessing",
            EnchantmentKind::Curse => "curse",
        }
    }
}

impl Into<String> for EnchantmentKind {
    fn into(self) -> String {
        match self {
            EnchantmentKind::Normal => "normal",
            EnchantmentKind::Blessing => "blessing",
            EnchantmentKind::Curse => "curse",
        }
        .to_string()
    }
}

impl From<&String> for EnchantmentKind {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "normal" => EnchantmentKind::Normal,
            "blessing" => EnchantmentKind::Blessing,
            "curse" => EnchantmentKind::Curse,
            _ => EnchantmentKind::Normal,
        }
    }
}

pub enum CardData {
    Yokai {
        max_health: i64,
        health: i64,
        damage: i64,
    },
    Artifact {},
    Terrain {},
    Enchantment {
        kind: EnchantmentKind,
    },
}

impl CardData {
    pub fn yokai(max_health: i64, health: i64, damage: i64) -> Self {
        Self::Yokai {
            max_health,
            health,
            damage,
        }
    }
    pub fn artifact() -> Self {
        Self::Artifact {}
    }
    pub fn terrain() -> Self {
        Self::Terrain {}
    }
    pub fn enchantment(kind: EnchantmentKind) -> Self {
        Self::Enchantment { kind }
    }
}

impl UserData for CardData {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        // Yokai
        add_card_field!(fields, Yokai, max_health, *max_health);
        add_card_field!(fields, Yokai, health, *health);
        add_card_field!(fields, Yokai, damage, *damage);

        // Enchantment
        add_card_field!(fields, Enchantment, kind, kind.as_str());
    }
}
