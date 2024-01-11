---@meta

---@class Game
---@field player fun(self: Game, player_id: number): Player
game = {}

---@class Player
---@field summon fun(self, slot: FieldSlot, card_type: CardType)
---@field draw fun(self)
---@field destroy fun(self, slot: FieldSlot)
--
-- Events
---@field select_slot fun(self, callback: fun(slot: FieldSlot))
