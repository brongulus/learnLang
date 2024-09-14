package main

import "testing"

// If you had initialized go mod with go mod init main you will be
// presented with an error _testmain.go:13:2: cannot import "main". This is
// because according to common practice, package main will only contain
// integration of other packages and not unit-testable code and hence Go
// will not allow you to import a package with name main.

func TestSum(t *testing.T) {
	numbers := [5]int{1, 2, 3, 4, 5}

	got := Sum(numbers)
	want := 15

	if got != want {
		t.Errorf("got %d want %d given, %v", got, want, numbers)
	}

}
