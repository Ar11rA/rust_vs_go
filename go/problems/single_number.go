package problems

// https://leetcode.com/problems/single-number/
// https://leetcode.com/problems/single-number-i/
func SingleNumberFind(nums []int) int {
	frequency := make(map[int]int)

	for _, value := range nums {
		if _, ok := frequency[value]; !ok {
			frequency[value] = 1
		} else {
			frequency[value] += 1
		}
	}

	for _, value := range frequency {
		if value == 1 {
			return value
		}
	}
	return 0
}

// https://leetcode.com/problems/single-number-iii/
func SingleNumberFilter(nums []int) []int {
	frequency := make(map[int]int)
	var results []int
	for _, value := range nums {
		if _, ok := frequency[value]; !ok {
			frequency[value] = 1
		} else {
			frequency[value] += 1
		}
	}

	for key, value := range frequency {
		if value == 1 {
			results = append(results, key)
		}
	}
	return results
}
