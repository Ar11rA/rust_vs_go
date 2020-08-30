package problems

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
var PhoneChars = []string{
	"abc",
	"def",
	"ghi",
}

func backtrack(digits string, result *[]string, store string, ctr int) {
	if ctr == len(digits) {
		*result = append(*result, store)
		return
	}
	mapStr := PhoneChars[digits[ctr]-'2']
	for i := 0; i < len(mapStr); i++ {
		backtrack(digits, result, store+string(mapStr[i]), ctr+1)
	}
}

func LetterCombinations(digits string) []string {
	var result []string
	backtrack(digits, &result, "", 0)
	return result
}
