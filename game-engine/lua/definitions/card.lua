---@meta

---@alias SpellKind
---| "normal"
---| "blessing"
---| "curse"

---@class YokaiData
---@field max_health integer
---@field health integer
---@field damage integer

---@class SpellData
---@field kind SpellKind

---@class Card
---@field activate fun(self, player: Player)?
