package problems

// https://leetcode.com/problems/binary-watch/ - Backtracking just the hours part
var hourConstants = [4]int{1, 2, 4, 8}

func backtrackHours(hours *[]int, num int, index int, result int) {
	if result > 11 {
		return
	}

	if num == 0 || index > 4 {
		*hours = append(*hours, result)
		return
	}
	for i := index; i < 4; i++ {
		backtrackHours(hours, num - 1, i + 1, result + hourConstants[i])
	}
}

func GetHours(num int) []int {
	var hours []int
	for i := 0; i <= num; i++ {
		backtrackHours(&hours, i, 0, 0)
	}
	return hours
}
