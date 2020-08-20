package problems

//https://leetcode.com/problems/find-and-replace-pattern/
func FindAndReplacePattern(words []string, pattern string) []string {
	var results []string
	for _, word := range words {
		if isSamePattern(word, pattern) {
			results = append(results, word)
		}
	}
	return results
}

func isSamePattern(word string, pattern string) bool {
	hash := make(map[rune]rune)

	if len(word) != len(pattern) {
		return false
	}
	isValid := true
	for index, char := range pattern {
		if _, ok := hash[char]; !ok {
			if includes(hash, rune(word[index])) {
				isValid = false
				break
			}
			hash[char] = rune(word[index])
		} else if rune(word[index]) != hash[char] {
			isValid = false
			break
		}
	}
	return isValid
}

func includes(hash map[rune]rune, char rune) bool {
	for _, v := range hash {
		if char == v {
			return true
		}
	}
	return false
}
