package problems

import "sort"

//https://leetcode.com/problems/custom-sort-string/
func Sort(S string, T string) string {
	sortCriteria := make(map[byte]int)
	for index := range S {
		sortCriteria[S[index]] = index
	}
	tBytes := []byte(T)
	sort.Slice(tBytes, func(i, j int) bool {
		return sortCriteria[T[i]] < sortCriteria[T[j]]
	})
	return string(tBytes)
}
