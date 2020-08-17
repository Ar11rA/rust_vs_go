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

    rectangle := problems.NewSubrectangle([][]int{{1,2}, {3,4}})
	log.Println("Subrectangle before", rectangle)
	rectangle.UpdateSubrectangle(0,0,1,1,5)
	log.Println("Subrectangle after", rectangle)

}
