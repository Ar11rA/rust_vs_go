package problems

func MaxProduct(nums []int) int {
	largest := 0
	secondLargest := 0
	for _, num := range nums {
		if num >= largest {
			largest, secondLargest = num, largest
		} else if num > secondLargest {
			secondLargest = num
		}
	}
	return (largest - 1) * (secondLargest - 1)
}
