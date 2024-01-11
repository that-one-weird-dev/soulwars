use mlua::{IntoLua, UserData};

macro_rules! add_card_field {
    ($fields:expr, $variant:ident, $field:ident, $value:expr) => {
        $fields.add_field_method_get("$field", |lua, this| match this {
            Card::$variant { $field, .. } => IntoLua::into_lua($value, lua),
            _ => Ok(mlua::Value::Nil),
        });
    };
}

pub enum SpellKind {
    Normal,
    Blessing,
    Curse,
}

impl SpellKind {
    fn as_str(&self) -> &'static str {
        match self {
            SpellKind::Normal => "normal",
            SpellKind::Blessing => "blessing",
            SpellKind::Curse => "curse",
        }
    }
}

impl Into<String> for SpellKind {
    fn into(self) -> String {
        match self {
            SpellKind::Normal => "normal",
            SpellKind::Blessing => "blessing",
            SpellKind::Curse => "curse",
        }
        .to_string()
    }
}

impl From<&String> for SpellKind {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "normal" => SpellKind::Normal,
            "blessing" => SpellKind::Blessing,
            "curse" => SpellKind::Curse,
            _ => SpellKind::Normal,
        }
    }
}

pub enum Card {
    Yokai {
        max_health: i64,
        health: i64,
        damage: i64,
    },
    Artifact {},
    Terrain {},
    Spell {
        kind: SpellKind,
    },
}

impl UserData for Card {
    fn add_fields<'lua, F: mlua::prelude::LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        // Yokai
        add_card_field!(fields, Yokai, max_health, *max_health);
        add_card_field!(fields, Yokai, health, *health);
        add_card_field!(fields, Yokai, damage, *damage);

        // Spell
        add_card_field!(fields, Spell, kind, kind.as_str());
    }
}
