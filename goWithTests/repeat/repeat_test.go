package iteration

import "testing"

// When the benchmark code is executed, it runs b.N times and measures how long it
// takes. The amount of times the code is run shouldn't matter to you, the framework will
// determine what is a "good" value for that to let you have some decent results.
// go test -bench=.

func BenchmarkRepeat(b *testing.B) {
	for range b.N {
		Repeat("a", 10)
	}
}

func TestRepeat(t *testing.T) {
	repeated := Repeat("a", 5)
	expected := "aaaaa"

	if repeated != expected {
		t.Errorf("expected %q got %q", expected, repeated)
	}
}
