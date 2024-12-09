package main

import "testing"

// go test (go mod init hello)

func TestHello(t *testing.T) {
	t.Run("saying hello to people", func(t *testing.T) {
		got := Hello("Chris", "English")
		want := "Hello, Chris"
		assertCorrectMessage(t, got, want)
	})
	t.Run("saying hello world for empty string", func(t *testing.T) {
		got := Hello("", "English")
		want := "Hello, World"
		assertCorrectMessage(t, got, want)
	})
	t.Run("in Spanish", func(t *testing.T) {
		got := Hello("Elodie", "Spanish")
		want := "Hola, Elodie"
		assertCorrectMessage(t, got, want)
	})
	t.Run("in French", func(t *testing.T) {
		got := Hello("Elodie", "French")
		want := "Bonjour, Elodie"
		assertCorrectMessage(t, got, want)
	})
}

// For helper functions, it's a good idea to accept a testing.TB which is an
// interface that *testing.T and *testing.B both satisfy, so you can call helper
// functions from a test, or a benchmark.

func assertCorrectMessage(t testing.TB, got, want string) {
	t.Helper() // reports linum on fail
	if got != want {
		t.Errorf("got %q want %q", got, want)
	}
}
