package main

import (
	"fmt"
	"time"
)

func printHello() {
	// time.Sleep(15 * time.Millisecond)
	fmt.Println("Hello")
}

func example1() {
	fmt.Println("main exec")

	go printHello()

	fmt.Println("main exec")
	time.Sleep(10 * time.Millisecond)
}

// --------------------------------------------

func main() {
	example1()
}
