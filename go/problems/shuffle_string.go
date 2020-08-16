package problems

// https://leetcode.com/problems/shuffle-string/
func RestoreString(s string, indices []int) string {
	res, tmp := []rune(s), []rune(s)
	for i := 0; i < len(indices); i++{
		res[indices[i]] = tmp[i]
	}
	return string(res)
}
