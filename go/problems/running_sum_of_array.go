package problems

// https://leetcode.com/problems/running-sum-of-1d-array/
func GetRunningSum(nums []int) []int {
	sums := make([]int, len(nums))
	sum := 0
	for i, num := range nums {
		sum = sum + num
		sums[i] = sum
	}
	return sums
}