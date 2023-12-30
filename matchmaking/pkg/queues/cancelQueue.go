package queues

import (
	"log"

	"google.golang.org/protobuf/proto"

	"soulwars-matchmaking/pkg/matchmaking"
	"soulwars-matchmaking/pkg/protos"
)

func consumeCancelQueue() error {
	cancelQueue, err := channel.QueueDeclare("matchmaking.cancel", false, false, false, false, nil)
	if err != nil {
		return err
	}

	messages, err := channel.Consume(
		cancelQueue.Name,
		"",
		true,
		false,
		false,
		false,
		nil,
	)
	if err != nil {
		return err
	}

	for message := range messages {
		request := &protos.MatchmakingCancel{}
		err = proto.Unmarshal(message.Body, request)
		if err != nil {
			return err
		}

		log.Print(" [*] Received cancel message")

		handleCancel(request)
	}

	return nil
}

func handleCancel(request *protos.MatchmakingCancel) {
	matchmaking.CancelMatchmakingForUser(request.UserId)
}
