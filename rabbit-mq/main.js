package main

import (
	"fmt"
	"log"
	"time"

	"github.com/streadway/amqp"
)

const (
	HOST = "localhost"
	PORT = 5672
)

type Queue struct {
	Name      string
	Durable   bool
	AutoDel   bool
	Exclusive bool
	NoWait    bool
}

// This is the publishing protocol that ampq uses
type PublishProtocol struct {
	ContentType     string    // MIME content type
	ContentEncoding string    // MIME content encoding
	DeliveryMode    uint8     // Transient (0 or 1) or Persistent (2)
	Priority        uint8     // 0 to 9
	CorrelationId   string    // correlation identifier
	ReplyTo         string    // address to to reply to (ex: RPC)
	Expiration      string    // message expiration spec
	MessageId       string    // message identifier
	Timestamp       time.Time // message timestamp
	Type            string    // message type name
	UserId          string    // creating user id - ex: "guest"
	AppId           string    // creating application id

	// The application specific payload of the message
	Body []byte
}

var testQueue = Queue{
	Name:      "testQueue",
	Durable:   false,
	AutoDel:   false,
	Exclusive: false,
	NoWait:    false,
}

func main() {
	// We need to connect to the rabbitMq virtual server
	var connString = fmt.Sprintf("amqp://guest:guest@%s:%d", HOST, PORT)
	conn, err := amqp.Dial(connString)
	if err != nil {
		log.Panicf("Could not connect to rabbit-mq:\n %s\n", err)
	}

	log.Println("Connection to RabbitMq initialised")
	defer conn.Close()

	ch, err := conn.Channel()
	if err != nil {
		log.Panicf("Could not create channel queue:\n %s\n", err)
	}
	defer ch.Close()

	q, err := ch.QueueDeclare(
		testQueue.Name,
		testQueue.Durable,
		testQueue.AutoDel,
		testQueue.Exclusive,
		testQueue.NoWait,
		nil,
	)

	if err != nil {
		log.Panicf("Error in creating channel:\n %s", err)
	}

	log.Println("New queue configured")
	fmt.Printf("\t%+v\n", q)

	body := "What data shouldn't be used in RabbitMQ? or what should it not be used for"
	errP := ch.Publish(
		"",
		testQueue.Name,
		false, false,
		amqp.Publishing{ContentType: "application/json", Body: []byte(body)},
	)
	panicOnErr(errP, "Could not publish message to queue")
	log.Println("[x] Published msg:", body)

}

func panicOnErr(e error, s string) {
	if e != nil {
		log.Panicf("%s\n :%s", s, e)
	}
}
