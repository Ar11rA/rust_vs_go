package problems

import "fmt"

func SimplifiedFractions(n int) []string {
  var results []string
  if n == 1 {
    return results
  }
  for i := 1; i <= n; i++ {
    for j := i + 1; j <= n; j++ {
      if gcd(j, i) == 1 {
        fraction := fmt.Sprintf("%d/%d", i, j)
        results = append(results, fraction)
      }
    }
  }
  return results
}

func gcd(a, b int) int {
  for b != 0 {
    a, b = b, a%b
  }
  return a
}
