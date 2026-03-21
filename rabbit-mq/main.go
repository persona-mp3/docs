package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"

	mq "github.com/rabbitmq/amqp091-go"
)

const (
	HOST = "localhost"
	PORT = 5672
)

var CONN_STRING = fmt.Sprintf("amqp://guest:guest@%s:%d", HOST, PORT)

func PanicErr(s string, err error) {
	if err != nil {
		fmt.Println(s)
		panic(err)
	}
}

func Publisher(queueName string) {
	conn, err := mq.Dial(CONN_STRING)
	PanicErr("Could not connect to rabbitMQ", err)
	defer conn.Close()
	// create channel with rabbit
	ch, err := conn.Channel()
	PanicErr("could not create channel with rabbit", err)
	defer ch.Close()

	// after channel, declare queue
	q, err := ch.QueueDeclare(queueName, false, false, false, false, nil)
	PanicErr("", err)
	// publish queue to rabbbit
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	body := "Maison Margilea Replica: Jazz Club"
	errP := ch.PublishWithContext(ctx, "", q.Name, false, false, mq.Publishing{
		ContentType: "text/application",
		Body:        []byte(body),
	})
	PanicErr("error: publishing message to channel", errP)

	log.Println("{*} Sent message to Rabbit")
	log.Printf("current-queue:\n %+v\n", q)
}

// EmulateTask() is used to assign process-intensive tasks to workers,
//
// See https://www.rabbitmq.com/tutorials/tutorial-two-go
func EmulateTask(queueName string, task string) {
	conn, err := mq.Dial(CONN_STRING)
	PanicErr("Could not connect to rabbitMQ", err)
	defer conn.Close()

	ch, err := conn.Channel()
	PanicErr("could not create channel with rabbit", err)
	defer ch.Close()

	q, err := ch.QueueDeclare("redeclared", false, false, false, false, nil)
	PanicErr("", err)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	body := task
	errP := ch.PublishWithContext(ctx, "", q.Name, false, false, mq.Publishing{
		DeliveryMode: mq.Persistent,
		ContentType:  "text/application",
		Body:         []byte(body),
	})
	PanicErr("error: publishing message to channel", errP)

	log.Println("{*} Sent message to Rabbit")
	log.Printf("current-queue:\n %+v\n", q)
}

func main() {
	// Publisher("maison-margilea")
	EmulateTask("maison-margilea", os.Args[1])
}
