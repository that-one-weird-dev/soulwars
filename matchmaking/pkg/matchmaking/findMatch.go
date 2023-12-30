package matchmaking

import (
	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/db/models"
)

func FindAvailableMatch() *models.Match {
    var match *models.Match
    err := db.Conn.Where("status = ?", models.MatchAvailable).Limit(1).First(&match).Error
    if err != nil {
	return nil
    }

    return match
}
