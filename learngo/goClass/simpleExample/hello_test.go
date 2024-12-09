package hello

import "testing"

// go test
// C-c C-t f in emacs

func TestSayHello(t *testing.T) {
	subtests := []struct {
		items  []string
		result string
	}{
		{
			result: "Hello, World!",
		},
		{
			items:  []string{"Matt"},
			result: "Hello, Matt!",
		},
		{
			items:  []string{"Matt, Anne"},
			result: "Hello, Matt, Anne!",
		},
	}

	for _, st := range subtests {
		if s := Say(st.items); s != st.result {
			t.Errorf("wanted %s (%v), got %s", st.result,
				st.items, s)
		}
	}
}
