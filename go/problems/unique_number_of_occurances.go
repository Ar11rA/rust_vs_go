package problems

// https://leetcode.com/problems/unique-number-of-occurrences/
func UniqueOccurrences(arr []int) bool {
	frequencyMap := make(map[int]int)
	for _, item := range arr {
		if _, ok := frequencyMap[item]; ok {
			frequencyMap[item] += 1
		} else {
			frequencyMap[item] = 1
		}
	}
	peek := make(map[int]bool)
	for _, value := range frequencyMap {
		if _, ok := peek[value]; ok {
			peek[value] = true
			return false
		}
	}
	return true
}
