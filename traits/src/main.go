package main

import (
	"fmt"
	"net"
	"sync"
)

type Msg struct {
	from string
	body string
}

type Res struct {
	body string
}

// / It's very hard to do this in rust with traits, especially with the whole Sync,
//
//	Send and stuff. But it kind of makes sense. It's not actually gauranteed that
//	whatever concrete type that implements this interface is actually safe for
//	concurrency use? Anyone could impl something that updates the internal state
//	if it wants to
type Transport interface {
	Connect(addr string, req Msg) (Res, error)
}

func broadcast(peers []string, transport Transport) {
	msg := Msg{}
	for _, addr := range peers {
		go transport.Connect(addr, msg)
	}
}

type SimTransport struct {
	// now add mutex for syncrhonsization
	mu       sync.Mutex
	requests map[string][]string
}

func (s *SimTransport) Connect(addr string, req Msg) (Res, error) {
	/// for example, diff go routines could update the .requests field
	// <lol, we actually have to properly initialise the map, but maps by default
	// aren't safe for concurrent access in go, so this will panic> but unless we
	// add a mu

	// but then the problem doesn't technically go away, i can decide not to take
	// the lock and still update a field as I see please. This can't be caught at
	// comptime, only at runtime _when_ you want to run the race detector
	//
	// SimTransport.mu says 'this type can be Sync (shared across other threads) if
	// you want to' I say Sync and not Send because these values can be accessed in
	// a synchronous way. But not Send, because we're using it in a pointer-receiver
	// method, This means that every routine can actively modify it without using the
	// Sync leading to data races. So send is implicit at the programmer level, but 
	// in Rust, it's shoved to your face
	return Res{body: "response"}, nil
}

type TcpTransport struct {
	// stateless
}

func (s *TcpTransport) Connect(addr string, req Msg) (Res, error) {
	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return Res{}, err
	}

	conn.Close()
	fmt.Println("closed connection succesfully")

	return Res{body: "response"}, nil
}

func main() {
	simTransport := &SimTransport{}
	broadcast([]string{"n2", "n3"}, simTransport)
}
