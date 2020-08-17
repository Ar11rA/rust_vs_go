package problems

//https://leetcode.com/problems/is-subsequence/
func IsSubsequence(s string, t string) bool {
	tIdx := 0
	ctr := 0
	for _, char := range t {
		if s[tIdx] == uint8(char) {
			ctr += 1
			tIdx += 1
		}
		if len(s) == ctr {
			return true
		}
	}
	return false
}
