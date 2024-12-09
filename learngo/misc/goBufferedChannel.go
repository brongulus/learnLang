package main

import "fmt"

func squares(c chan int) {

	for val := range c {
		fmt.Println(val * val)
	} // goes till 16? HOW?

	// for i := 0; i <= 3; i++ {
	// 	num := <-c // Buffered Reads are draining i.e. all the reads would be non-blocg except when i == 3, then theres nothing in the channel hence it moves to main c <- 4 write. WRONG!
	// 	fmt.Println(num * num)
	// }
}

func main() {
	fmt.Println("main() started")
	c := make(chan int, 3)
	c2 := make(chan int, 3)

	go squares(c)
	go squares(c2)

	// When the buffer size is non-zero n, goroutine is not blocked until after buffer is full. When
	// the buffer is full, any value sent to the channel is added to the buffer by throwing out last
	// value in the buffer which is available to read (where the goroutine will be blocked). But there
	// is a catch, read operation on buffered is thirsty. That means, once read operation starts, it
	// will continue until the buffer is empty. Technically, that means goroutine that reads buffer
	// channel will not block until the buffer is empty.

	c <- 1
	c <- 2
	c <- 3
	c <- 4 // blocks here
	c2 <- 5
	c2 <- 6
	c2 <- 7
	c2 <- 8 // blocks here

	fmt.Println("main() stopped")
}
