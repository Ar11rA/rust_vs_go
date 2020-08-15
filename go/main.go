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
}
