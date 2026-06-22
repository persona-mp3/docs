package main

import (
	"fmt"
	// uncomment_2_
	// "sync"
)

func goroutine() {
	for i := range 10 {
		fmt.Println("from_routineB: ", i)
	}
}

func main() {

	// __1__
	// well, my guess is because the main thread is scheduled to run first
	// Well, threads have three states, RUNNING, READY, BLOCKED
	//
	// When this program is ran, we have the main goroutine who and the
	// other go-routine we spawned who are both READY. These are added to a queue we can
	// call the `ready_queue`
	//
	// The scheulder decideds on which routine to run first, by looking at
	// at the ready-queue. The next routine is ran when
	// 1. The current routine is ran till completion
	// 2. It yeilded to the scheudler
	// 3. It is blocked waiting for IO, or instructions from main memory
	//
	//
	// Our entry program is main() and that routine is added first to the ready_queue, followed
	// by the second goroutine spawned with `go`
	//
	// The for-loop in the first one runs immediately. The problem is that it runs in the main function.
	// So by the time the main function has exited, the second goroutine had no chance to run (starved),
	// and ends up getting terminated by the runtime, even though it was in the ready_queue and was READY

	// __2__
	// But if we used some sort of coordination primitive ie WaitGroup, which is similar to
	// c's wait() procedure, we achieve a blocking state. So when this function runs, we add
	// a goroutine to the waitgroup. This increases and internal counter to track the number
	// of active goroutines spawned. And then towards the end of main, we call wg.Wait() which
	// doesn't stop blocking until the internal counter is `0` which is why we call `defer wg.Done()`
	// inside the anonymous function. How the scheduler reacts to this is that, if it notices
	// that a routine isn't making progress, it switches to another routine to run that. However, the
	// goroutine might also voluntarily yield to the scheduler instead of the scheduler interrupting it
	// and running another routine (pre-emptive scheudling)

	// We could also make the second goroutine run in another way by purposely blocking or yeilding.
	// For example, instead of using a waitgroup, we could use a `time.Sleep(x * time.Seconds)`. This
	// will put the routine in a BLOCKED state, which will cause the routine to yield , and then the  
	// scheduler can switch to the next routine till they both exit or finish

	// uncomment_2_
	// wg := sync.WaitGroup{}
	// wg.Add(1)

	// uncomment_2_
	// go func(wg *sync.WaitGroup) {
	// 	defer wg.Done()
	// 	goroutine()
	// }(&wg)

	// _1_
	go goroutine()

	for i := range 10 {
		fmt.Println("from_main: ", i)
	}

	// uncommnet_2_
	// wg.Wait()
}
