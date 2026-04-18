package main

import (
	"bytes"
	"fmt"
	"log"
	"time"

	mq "github.com/rabbitmq/amqp091-go"
)

const (
	HOST = "localhost"
	PORT = 5672
)

var CONN_STRING = fmt.Sprintf("amqp://guest:guest@%s:%d", HOST, PORT)

type mqConn struct {
	*mq.Connection
	q *mq.Queue
}

func Connect(queueName string) {
	conn, err := mq.Dial(CONN_STRING)
	if err != nil {
		fmt.Println("C: could not connect to rabbitmq")
		panic(err)
	}

	ch, err := conn.Channel()
	if err != nil {
		fmt.Println("C: could not create channel")
		panic(err)
	}
	defer conn.Close()
	defer ch.Close()

	// setting up the queue we want to listen for
	q, err := ch.QueueDeclare(
		queueName, false, false, false, false, nil,
	)
	PanicError("C: could not declear queue", err)

	// consume messages fromn the connected channel
	msgs, err := ch.Consume(
		q.Name, "", true, false, false, false, nil,
	)
	var forever = make(chan bool)
	go func() {
		for n := range msgs {
			fmt.Println("\n\t\t[*] Notification: ")
			log.Printf("%s\n", n.Body)
		}
	}()

	fmt.Println("---Waiting for new messages")
	<-forever

}

func PanicError(s string, err error) {
	if err != nil {
		fmt.Println(s)
		panic(err)
	}
}

// DoTask() is used to process-intensive tasks as a worker,
//
// See https://www.rabbitmq.com/tutorials/tutorial-two-go
func DoTask(queueName string) {
	conn, err := mq.Dial(CONN_STRING)
	if err != nil {
		fmt.Println("C: could not connect to rabbitmq")
		panic(err)
	}

	ch, err := conn.Channel()
	if err != nil {
		fmt.Println("C: could not create channel")
		panic(err)
	}
	defer conn.Close()
	defer ch.Close()

	q, err := ch.QueueDeclare(
		queueName, false, false, false, false, nil,
	)
	PanicError("C: could not declear queue", err)

	msgs, err := ch.Consume(
		q.Name, "", true, false, false, false, nil,
	)
	var forever = make(chan bool)
	go func() {
		for n := range msgs {
			fmt.Println("\n\t\t[*] Notification: ")
			log.Printf("%s\n", n.Body)
			fmt.Println("counting bytes....")
			dotCount := bytes.Count(n.Body, []byte("."))

			t := time.Duration(dotCount)
			time.Sleep(t * time.Second)
			log.Printf("Done executing task ->")
			// n.Ack()
		}
	}()

	fmt.Println("---Waiting for new messages")
	<-forever

}

func main() {
	// Connect("maison-margilea")
	DoTask("maison-margilea")

}
