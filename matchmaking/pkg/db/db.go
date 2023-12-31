package db

import (
	"errors"
	"os"
	"soulwars-matchmaking/pkg/db/models"

	libsql "github.com/ekristen/gorm-libsql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var Conn *gorm.DB

func Init() error {
    var err error

    db_url := os.Getenv("DATABASE_MATCHMAKING_URL")
    if db_url == "" {
	return errors.New("No database url")
    }
    auth_token := os.Getenv("DATABASE_MATCHMAKING_AUTH_TOKEN")
    if auth_token == "" {
	return errors.New("No database auth token")
    }

    Conn, err = gorm.Open(libsql.Open(db_url + "?authToken=" + auth_token), &gorm.Config{
	Logger: logger.Default.LogMode(logger.Silent),
    })
    if err != nil {
        return err
    }

    Conn.AutoMigrate(&models.Match{})

    return nil
}
