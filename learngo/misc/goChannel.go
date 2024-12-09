package main

import (
	"fmt"
	"runtime"
)

func test(c chan int) {
	fmt.Printf("Test: channel has %v\n", <-c) // After c <- 1 in main, reads from c are blocking only if there's nothing written to c
	c <- 2                                    // Blocks test, data written in c, main becomes active
	fmt.Printf("Test: channel has %v\n", <-c) // After c <- 3 in main
	close(c)                                  // Closing channel, no more writes to channel allowed, however reads from it are still possible, close is non-blocking operation
	fmt.Println("Test: after close(c)")
}

func main() {
	c := make(chan int)
	receiveOnlyChannel := make(<-chan int)
	sendOnlyChannel := make(chan<- int)

	fmt.Printf("Main: type of `c` is %T\n", c)
	fmt.Println("Main: value of `c` is ", c) // Channels are sort of a pointer
	fmt.Printf("Main: type of `receiveOnlyChannel` is %T\n", receiveOnlyChannel)
	fmt.Printf("Main: type of `sendOnlyChannel` is %T\n", sendOnlyChannel)

	fmt.Println("---------------------------------------------")

	go test(c) // Active goroutine is main

	// When some data is written to the channel ("send" to channel), goroutine is blocked until some
	// other goroutine reads it ("receive" from channel) from that channel. Channel operations tell the
	// scheduler to schedule another goroutine, that’s why a program doesn’t block forever on the same
	// goroutine.

	c <- 1 // Main gets blocked as data written in c, test gets scheduled

	// If you are trying to read data from a channel but channel does not have a value available with it,
	// it blocks the current goroutine and unblocks other in a hope that some goroutine will push a value
	// to the channel. Hence, this read operation will be blocking.
	// Similarly, if you are to send data to a channel, it will block current goroutine and unblock
	// others until some goroutine reads the data from it. Hence, this send operation will be blocking.

	data := <-c // Main is active (after c <- 2 in test)
	fmt.Println("Main: value of `data` is ", data)

	fmt.Println("active goroutines ", runtime.NumGoroutine())
	c <- 3 // Main blocked, test is active

	fmt.Println("Main: This is executing sometimes before `val, ok := <-c` and sometimes afterwards") // Eh, WAT?!
	fmt.Println("active goroutines ", runtime.NumGoroutine())

	val, ok := <-c // After test execution is over

	fmt.Println("active goroutines after val := <-c ", runtime.NumGoroutine())
	if ok == false {
		fmt.Println("Main: channel is closed! And has value ", val)
	}
}
