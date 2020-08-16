package problems

// https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time
func BusyStudent(startTime []int, endTime []int, queryTime int) int {
	result := 0
	for i, v := range startTime {
		if v <= queryTime && queryTime <= endTime[i] {
			result+= 1
		}
	}
	return result
}
