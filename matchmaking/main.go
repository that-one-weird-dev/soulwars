package main

import (
	"log"

	"github.com/joho/godotenv"

	"soulwars-matchmaking/pkg/db"
	"soulwars-matchmaking/pkg/queues"
)

func main() {
	godotenv.Load()

	err := db.Init()
	if err != nil {
		panic(err)
	}

	queues.Run()

	log.Printf(" [*] Waiting for messages. To exit press CTRL+C")
	var forever chan struct{}
	<- forever
}
