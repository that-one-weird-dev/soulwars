package queues

import (
	"context"
	"log"
	"time"

	amqp "github.com/rabbitmq/amqp091-go"
	"google.golang.org/protobuf/proto"

	"soulwars-matchmaking/pkg/protos"
)

func publishMatchFound(userId string, replyTo string) error {
	log.Println(" [*] Sending match found on queue " + replyTo)

	response := &protos.MatchmakingFound{
		UserId:  userId,
		MatchId: "",
	}

	responseBytes, err := proto.Marshal(response)
	if err != nil {
		return err
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	channel.PublishWithContext(
		ctx,
		"",
		replyTo,
		false,
		false,
		amqp.Publishing{
			ContentType: "text/plain",
			Body:        responseBytes,
		},
	)

	return nil
}
