package main

import "fmt"

// * Write a test
// * Make the compiler pass
// * Run the test, see that it fails and check the error message is meaningful
// * Write enough code to make the test pass
// * Refactor

const englishHelloPrefix = "Hello, "
const spanishHelloPrefix = "Hola, "
const frenchHelloPrefix = "Bonjour, "

func Hello(name, language string) string {
	if name == "" {
		name = "World"
	}

	return greetingPrefix(language) + name
}

// The function name starts with a lowercase letter. In Go public functions start
// with a capital letter and private ones start with a lowercase. We don't want
// the internals of our algorithm to be exposed to the world, so we made this
// function private.

func greetingPrefix(language string) (prefix string) {
	switch language {
	case "French":
		prefix = frenchHelloPrefix
	case "Spanish":
		prefix = spanishHelloPrefix
	default:
		prefix = englishHelloPrefix
	}
	return
}

// go run hello.go

func main() {
	name := "Chris"
	language := "English"
	fmt.Println(Hello(name, language))
}
