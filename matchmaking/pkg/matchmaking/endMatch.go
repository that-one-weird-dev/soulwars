package matchmaking

import (
	"github.com/google/uuid"

	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/db/models"
)

func EndMatch(gameId uuid.UUID) {
	db.Conn.Delete(&models.Match{
		GameID: gameId,
	})
}
