package problems

import (
	"sort"
)

// https://leetcode.com/problems/search-suggestions-system
func SuggestedProducts(products []string, searchWord string) [][]string {
	sort.Strings(products)
	dest := make([][]string, len(searchWord)+1)
	dest[0] = products
	for i, c := range searchWord {
		for _, s := range dest[i] {
			if len(s) > i && s[i] == byte(c) {
				dest[i+1] = append(dest[i+1], s)
			}
		}
	}
	for i := 1; i < len(dest); i++ {
		if len(dest[i]) > 3 {
			dest[i] = dest[i][:3]
		}
	}
	return dest[1:]
}
