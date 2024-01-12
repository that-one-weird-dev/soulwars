use mlua::{FromLua, IntoLua, Lua, Value};

pub enum FieldSlot {
    Yokai1,
    Yokai2,
    Yokai3,
    Artifact1,
    Artifact2,
    Artifact3,
    Terrain,
    Enchantment,
}

impl FieldSlot {
    fn to_str(&self) -> &'static str {
        match self {
            FieldSlot::Yokai1 => "yokai-1",
            FieldSlot::Yokai2 => "yokai-2",
            FieldSlot::Yokai3 => "yokai-3",
            FieldSlot::Artifact1 => "artifact-1",
            FieldSlot::Artifact2 => "artifact-2",
            FieldSlot::Artifact3 => "artifact-3",
            FieldSlot::Terrain => "terrain",
            FieldSlot::Enchantment => "enchantment",
        }
    }
}

impl TryFrom<&str> for FieldSlot {
    type Error = mlua::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "yokai-1" => Ok(Self::Yokai1),
            "yokai-2" => Ok(Self::Yokai2),
            "yokai-3" => Ok(Self::Yokai3),
            "artifact-1" => Ok(Self::Artifact1),
            "artifact-2" => Ok(Self::Artifact2),
            "artifact-3" => Ok(Self::Artifact3),
            "terrain" => Ok(Self::Terrain),
            "enchantment" => Ok(Self::Enchantment),
            _ => Err(mlua::Error::runtime("Invalid field slot")),
        }
    }
}

impl<'lua> FromLua<'lua> for FieldSlot {
    fn from_lua(value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
        match value {
            Value::String(value) => TryInto::<FieldSlot>::try_into(value.to_str()?),
            _ => Err(mlua::Error::runtime("Field slot must be a string")),
        }
    }
}

impl<'lua> IntoLua<'lua> for FieldSlot {
    fn into_lua(self, lua: &'lua Lua) -> mlua::prelude::LuaResult<Value<'lua>> {
        lua.create_string(self.to_str()).map(Value::String)
    }
}
