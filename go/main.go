package main

import (
  "log"

  "rust_vs_go/go/problems"
)

func main() {
  nums := []int{1, 2, 3, 4}
  log.Println("Running sum problem", problems.GetRunningSum(nums))

  candies := []int{2, 3, 5, 1, 3}
  extraCandies := 3
  log.Println("Extra candies problem", problems.KidsWithCandies(candies, extraCandies))

  inputStr := "codeleet"
  indices := []int{4, 5, 6, 7, 0, 2, 1, 3}
  log.Println("Shuffle string problem", problems.RestoreString(inputStr, indices))

  num := 234
  log.Println("Product minus sum problem", problems.SubtractProductAndSum(num))

  tree := problems.TreeNode{
    Val:   0,
    Left:  nil,
    Right: nil,
  }
  log.Println("BST range sum problem", problems.RangeSumBST(&tree, 1, 10))

  startTime := []int{1, 2, 3}
  endTime := []int{3, 2, 7}
  log.Println("Homework at query time problem", problems.BusyStudent(startTime, endTime, 4))

  arr := []int{1, 2, 2, 1, 1, 3}
  log.Println("Unique number of occurances", problems.UniqueOccurrences(arr))

  title := "AAB"
  log.Println("Letter tile possibilties", problems.NumTilePossibilities(title))

  searchNums := []int{1, 3, 5, 6}
  target := 0
  log.Println("Search insert position", problems.SearchInsert(searchNums, target))

  s := "ace"
  t := "abcde"
  log.Println("Is subsequence problem", problems.IsSubsequence(s, t))

  nums1 := []int{1, 3, 5}
  nums2 := []int{2, 5, 6}
  log.Println("Merge two sorted arrays", problems.Merge(nums1, 3, nums2, 3))

  nums3 := []int{1, 2, 3, 4, 5}
  log.Println("Rotate Array", problems.Rotate(nums3, 2))

  rectangle := problems.NewSubrectangle([][]int{{1, 2}, {3, 4}})
  log.Println("Subrectangle before", rectangle)
  rectangle.UpdateSubrectangle(0, 0, 1, 1, 5)
  log.Println("Subrectangle after", rectangle)

  words := []string{"abc", "deq", "mee", "aqq", "dkd", "ccc"}
  pattern := "abb"
  log.Println("Find and replace pattern", problems.FindAndReplacePattern(words, pattern))

  nums4 := []int{2, 1, 2, 1, 2, 1, 99}
  log.Println("Single number problem 1", problems.SingleNumberFind(nums4))
  log.Println("Single number problem 2", problems.SingleNumberFilter(nums4))

  cashier := problems.Constructor(3, 50, []int{1, 2, 3, 4, 5, 6, 7}, []int{100, 200, 300, 400, 300, 200, 100})
  log.Println("Cashier problem run 1", cashier.GetBill([]int{1, 2}, []int{1, 2}))
  log.Println("Cashier problem run 2", cashier.GetBill([]int{3, 7}, []int{10, 10}))
  log.Println("Cashier problem run 3", cashier.GetBill([]int{1, 2, 3, 4, 5, 6, 7}, []int{1, 1, 1, 1, 1, 1, 1}))
  log.Println("Cashier problem run 4", cashier.GetBill([]int{4}, []int{10}))

  log.Println("Custom sort string", problems.Sort("cba", "abcd"))

  log.Println("Seacrh suggestions system", problems.SuggestedProducts([]string{"cba", "cbad"}, "cb"))

  log.Println("Remove invalid parenthesis", problems.MinRemoveToMakeValid("())(()"))

  log.Println("Binary watch problem", problems.GetHours(2))

  log.Println("Letter combination problem", problems.LetterCombinations("243"))

  log.Println("Product of 2 largest problem", problems.MaxProduct([]int{2, 4, 5, 3}))

  log.Println("Generate parenthesis problem", problems.GenerateParenthesis(3))

  log.Println("Simplified fractions problem", problems.SimplifiedFractions(4))

  log.Println("Trie prefix tree problem")
  trie := problems.NewTrie()
  trie.Insert("yoda")
  trie.Insert("yoma")
  trie.Insert("yo")
  log.Println(trie)

}
