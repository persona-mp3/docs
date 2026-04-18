package main

import (
	"fmt"
	"os"
	adpb "tuts/protocols/data"

	"google.golang.org/protobuf/proto"
)

func main() {

	p1 := &adpb.Person{
		Name:  "Celtic",
		Id:    32,
		Email: "celticsprings@co.uk",
	}

	b, err := proto.Marshal(p1)
	if err != nil {
		fmt.Println("Error occured trying to marshal protobuf\n", err)
	}

	if err := os.WriteFile("user-bin", b, 0644); err != nil {
		fmt.Printf("an error occured writing to user-bin\n%s", err)
	}
}
