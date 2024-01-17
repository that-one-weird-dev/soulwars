
---@class Card
---@field data YokaiData
local card = {}

function card:activate(player)

	player:select_slot(function(slot)
		player:summon(slot, self.id)
	end)
end

create_card(1, card)
