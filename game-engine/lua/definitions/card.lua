---@meta

---@alias EnchantmentKind
---| "normal"
---| "blessing"
---| "curse"

---@class YokaiData
---@field max_health integer
---@field health integer
---@field damage integer

---@class EnchantmentData
---@field kind EnchantmentKind

---@class Card
---@field id integer
---@field activate fun(self, player: Player)?
