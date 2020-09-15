package problems

import (
  "sort"
  "strings"
)

// https://leetcode.com/problems/reorder-data-in-log-files
func ReorderLogFiles(logs []string) []string {

  var digitLogs []string
  var letterLogs []string

  for _, v := range logs {

    temp := strings.Fields(v)
    tmp := []rune(temp[1])

    if tmp[0] < 'a' || tmp[0] > 'z' {
      digitLogs = append(digitLogs, v)
      continue
    }
    letterLogs = append(letterLogs, v)
  }

  sort.SliceStable(letterLogs, func(i, j int) bool {

    iIndex := strings.Index(letterLogs[i], " ")
    jIndex := strings.Index(letterLogs[j], " ")

    iLog := letterLogs[i][iIndex:]
    jLog := letterLogs[j][jIndex:]

    if iLog == jLog {
      return letterLogs[i] < letterLogs[j]
    }
    return iLog < jLog
  })

  return append(letterLogs, digitLogs...)

}
