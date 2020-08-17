package problems

// https://leetcode.com/problems/letter-tile-possibilities/
func dfs(letters map[rune]int) int {
	sum := 0
	for k, v := range letters {
		if v == 0 {
			continue
		}

		sum++

		letters[k]--
		sum += dfs(letters)
		letters[k]++
	}

	return sum
}

func NumTilePossibilities(tiles string) int {
	letters := map[rune]int{}
	for _, r := range tiles {
		letters[r]++
	}

	return dfs(letters)
}
