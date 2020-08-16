package problems

// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
func SubtractProductAndSum(n int) int {
	temp := n
	prod := 1
	sum := 0
	for temp > 0 {
		digit := temp % 10
		sum += digit
		prod *= digit
		temp = temp / 10
	}
	return prod - sum
}
