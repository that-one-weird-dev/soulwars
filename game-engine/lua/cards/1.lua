
---@class Card
---@field data YokaiData
local card = {}

function card:activate(player)

	player:select_slot(function()
		player:draw()
	end)
end

return card
