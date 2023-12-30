package matchmaking

import (
	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/db/models"
)

func SetMatchOngoing(match *models.Match, userId, replyTo string) {
	match.User2ID = userId
	match.ReplyTo2 = replyTo
	match.Status = models.MatchOngoing

	db.Conn.Save(match)
}
