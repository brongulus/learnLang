package iteration

func Repeat(c string, times int) string {
	var repeatedString string

	for i := 0; i < times; i++ {
		repeatedString += c
	}

	return repeatedString
}
