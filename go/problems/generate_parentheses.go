package problems

import "log"

// https://leetcode.com/problems/generate-parentheses
func GenerateParenthesis(n int) []string {
  var results []string
  backtrackBrackets(&results, "", 0,0, n * 2)
  return results
}

func backtrackBrackets(acc *[]string, result string, open int, close int, max int) {
  log.Println(result)
  if max == len(result) {
    *acc = append(*acc, result)
    return
  }

  if open < max /2 {
    backtrackBrackets(acc, result+"(", open+1, close, max)
  }
  if close < open {
    backtrackBrackets(acc, result+")", open, close+1, max)
  }
}
