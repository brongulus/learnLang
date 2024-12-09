package main

import (
	"fmt"
	"hello"
	"os"
)

// go run ../cmd
// go run ../cmd Matt

func main() {
	// [1:] slice of strings not including the first item
	// An empty slice is valid as well
	fmt.Println(hello.Say(os.Args[1:]))
}
