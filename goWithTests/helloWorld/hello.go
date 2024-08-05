package main

import "fmt"

func Hello(name string) string {
	return "Hello, " + name
}

func main() {
	name := "Chris"
	fmt.Println(Hello(name))
}
