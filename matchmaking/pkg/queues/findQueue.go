package queues

import (
	"log"

	"google.golang.org/protobuf/proto"

	"soulwars-matchmaking/pkg/matchmaking"
	"soulwars-matchmaking/pkg/protos"
)

func consumeFindQueue() error {
	findQueue, err := channel.QueueDeclare("matchmaking.find", false, false, false, false, nil)
	if err != nil {
		return err
	}

	messages, err := channel.Consume(
		findQueue.Name,
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
		request := &protos.MatchmakingFind{}
		err = proto.Unmarshal(message.Body, request)
		if err != nil {
			return err
		}

		log.Print(" [*] Received find message")

		handleFind(request, message.ReplyTo)
		if err != nil {
			return err
		}
	}

	return nil
}

func handleFind(request *protos.MatchmakingFind, replyTo string) error {
	match := matchmaking.FindAvailableMatch()

	if match == nil {
		matchmaking.CreateMatch(request.UserId, replyTo)
		return nil
	} else {
		matchmaking.SetMatchOngoing(match, request.UserId, replyTo)

		err := publishMatchFound(match.UserID, match.ReplyTo, match.GameID.String())
		if err != nil {
			return err
		}

		return publishMatchFound(match.User2ID, match.ReplyTo2, match.GameID.String())
	}
}
