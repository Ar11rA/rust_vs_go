package problems

// https://leetcode.com/problems/merge-sorted-array/
func Merge(nums1 []int, m int, nums2 []int, n int) []int {
	var result []int
	ctr1 := 0
	ctr2 := 0
	for i := 0; i < m + n; i++ {
		if ctr1 == m {
			result = append(result, nums2[ctr2:] ...)
			break
		}

		if ctr2 == n {
			result = append(result, nums1[ctr1:] ...)
			break
		}

		if nums1[ctr1] < nums2[ctr2] {
			result = append(result, nums1[ctr1])
			ctr1 += 1
		} else {
			result = append(result, nums2[ctr2])
			ctr2 += 1
		}
	}
	return result
}
