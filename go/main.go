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

	inputStr:= "codeleet"
	indices := []int{4, 5, 6, 7, 0, 2, 1, 3}
	log.Println("Shuffle string problem", problems.RestoreString(inputStr, indices))

	num := 234
	log.Println("Product minus sum problem", problems.SubtractProductAndSum(num))

	tree := problems.TreeNode{
		Val:   0,
		Left:  nil,
		Right: nil,
	}
	log.Println("BST range sum problem", problems.RangeSumBST(&tree, 1, 10));
}
