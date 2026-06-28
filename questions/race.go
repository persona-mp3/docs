package main

import (
	"fmt"
	"sync"
)

/*
The thing is that for a datarace, one thread must be writing
and the other reading it, relatively at the same time, but
what happens if it's just a show glass? It's still a datarace but there's
no problem if it's only reads? So sematics like `const` in some sense, SHOULD gaurantee
no further writes can be made to that value it's pointing to

So I am going to assume that the default types are not concurrent safe for access, so
I expect the panic to be verbose here, I typically want to test it against the slice

But also, I've only seen sync.Map and not sync.NewSlice which I could infer that slices
could be safe for concurrent access but not modification, but another issue could be
the strings too

__1__
But now if we modify any value, it panics
SO i was right, no modification, no panic even when ran with --race

__2__
What if we create a new slice and realloc?
And yes it does panic, So neither are safe for concurrent use, but is it possible to 
do something like this
	if this value meets this condition, it should be const!

	So it becomes READONLY like Object.Freezez or if I remmber correctly in Kotlin
	the `val` keyword? You can change the underlying object it points to but you 
	can't change what it points to. Scary

*/

func main() {
	s := []string{
		"regular show",
		"mordecai",
		"rigby",
	}
	wg := sync.WaitGroup{}
	wg.Add(1)

	go func() {
		defer wg.Done()
		for idx, rs := range s[:] {
			// __1__
			// if idx%2 == 0 {
			// 	s[idx] = "cause data race"
			// }
			 _ = idx
			// __2__
			// slice := make([]string, len(s))
			// slice = append(slice, s...)
			// s = slice
			fmt.Println("routine::", rs)
		}
	}()

	for _, i := range s {
		fmt.Println("main::", i)
	}

	wg.Wait()
	fmt.Println("goneahthedays")
}
