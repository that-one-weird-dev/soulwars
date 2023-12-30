package queues

import (
	"os"

	amqp "github.com/rabbitmq/amqp091-go"
)

var channel *amqp.Channel

func Run() error {
	var err error
	conn, err := amqp.Dial(os.Getenv("RABBIT_URL"))
	if err != nil {
		return err
	}

	channel, err = conn.Channel()
	if err != nil {
		return err
	}

	go consumeFindQueue()
	go consumeCancelQueue()

	return nil
}
