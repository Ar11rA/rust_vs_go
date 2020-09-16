package problems

// https://leetcode.com/problems/detect-capital/
func DetectCapitalUse(word string) bool {
  allUpperCase := true
  for _, letter := range word {
    if !isUpperCase(letter) {
      allUpperCase = false
    }
  }
  if allUpperCase {
    return true
  }
  allLowerCase := true
  for _, letter := range word {
    if !isLowerCase(letter) {
      allLowerCase = false
    }
  }
  if allLowerCase {
    return true
  }
  firstUpperCase := true
  for index, letter := range word {
    if index == 0 {
      if !isUpperCase(letter) {
        firstUpperCase = false
      }
    } else {
      if !isLowerCase(letter) {
        firstUpperCase = false
      }
    }
  }
  if firstUpperCase {
    return true
  }
  return false
}

func isUpperCase(i int32) bool {
  if i >= 'A' && i <= 'Z' {
    return true
  }
  return false
}

func isLowerCase(i int32) bool {
  if i >= 'a' && i <= 'z' {
    return true
  }
  return false
}
