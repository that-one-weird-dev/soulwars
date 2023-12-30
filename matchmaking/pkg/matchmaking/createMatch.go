package matchmaking

import (
	"github.com/google/uuid"

	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/db/models"
)

func CreateMatch(userId, replyTo string) uuid.UUID {
	gameId := uuid.New()
	newMatch := &models.Match{
		GameID: gameId,
		UserID: userId,
		ReplyTo: replyTo,
		Status: models.MatchAvailable,
	}

	db.Conn.Create(&newMatch)

	return gameId
}
