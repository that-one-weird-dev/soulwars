package matchmaking

import (
	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/db/models"
)

func CancelMatchmakingForUser(userId string) {
	db.Conn.
		Where("user_id = ?", userId).
		Where("status = ?", models.MatchAvailable).
		Delete(&models.Match{})
}
