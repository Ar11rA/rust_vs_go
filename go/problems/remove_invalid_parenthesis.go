package problems

import "strings"

type stack []int

func (s *stack) push(element int) {
	*s = append(*s, element)
}

func (s *stack) pop() int {
	if len(*s) == 0 {
		return -1
	}
	last := (*s)[len(*s)- 1]
	*s = (*s)[:len(*s)-1]
	return last
}

// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses
func MinRemoveToMakeValid(s string) string {
	bracketStack := make(stack, 0)
	bytes := []byte(s)
	for index, char := range s {
		if char == '(' {
			bracketStack.push(index)
		} else if char == ')' {
			popped := bracketStack.pop()
			if popped == -1 {
				bytes[index] = ' '
			}
		}
	}
	for {
		popped := bracketStack.pop()
		if popped == -1 {
			break
		}
		bytes[popped] = ' '
	}
	return strings.Replace(string(bytes), " ", "", -1)
}