package problems

// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
func KidsWithCandies(candies []int, extraCandies int) []bool {
	threshold := findMax(candies)
	candiesWithExtraOnes := make([]bool, len(candies))
	for idx, candy := range candies {
		finalSum := candy + extraCandies
		if finalSum >= threshold {
			candiesWithExtraOnes[idx] = true
		} else {
			candiesWithExtraOnes[idx] = false
		}
	}
	return candiesWithExtraOnes
}

func findMax(candies []int) int {
	max := candies[0]
	for _, candy := range candies {
		if candy > max {
			max = candy
		}
	}
	return max
}