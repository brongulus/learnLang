package hello

import "strings"

func Say(name []string) string {
	if len(name) == 0 {
		name = []string{"World"}
	}
	return "Hello, " + strings.Join(name, ", ") + "!"
}
