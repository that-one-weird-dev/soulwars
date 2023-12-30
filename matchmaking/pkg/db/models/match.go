package models

import (
	"github.com/google/uuid"
)

const (
	MatchAvailable uint8 = 0
	MatchOngoing         = 1
)

type Match struct {
	GameID   uuid.UUID `gorm:"primaryKey"`
	UserID   string
	User2ID  string
	ReplyTo  string
	ReplyTo2 string
	Status   uint8 `gorm:"index"`
}
