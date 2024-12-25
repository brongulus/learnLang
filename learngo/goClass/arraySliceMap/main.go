package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

func main() {
	scan := bufio.NewScanner(os.Stdin)
	words := make(map[string]int)

	scan.Split(bufio.ScanWords)

	for scan.Scan() {
		words[scan.Text()]++
	}

	fmt.Println(len(words), "unique words")

	type kv struct {
		key string
		val int
	}

	var sortedSlice []kv

	for k, v := range words {
		sortedSlice = append(sortedSlice, kv{k, v})
	}

	sort.Slice(sortedSlice, func(i, j int) bool {
		return sortedSlice[i].val > sortedSlice[j].val // descending
	})

	for _, s := range sortedSlice {
		fmt.Println(s.key, "appears", s.val, "times")
	}
}
