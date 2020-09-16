package problems

// https://leetcode.com/problems/reverse-only-letters/
func ReverseOnlyLetters(S string) string {
  res := []byte(S)
  i, j := 0, len(S)-1
  for i <= j {
    switch {
    case isLetter(res[i]) && isLetter(res[j]):
      res[i], res[j] = res[j], res[i]
      i++
      j--
    case isLetter(res[i]):
      j--
    case isLetter(res[j]):
      i++
    default:
      i++
      j--
    }
  }
  return string(res)
}

func isLetter(i uint8) bool {
  if (i >= 'a' && i <= 'z') || (i >= 'A' && i <= 'Z') {
    return true
  }
  return false
}
