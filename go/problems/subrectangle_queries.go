package problems

// https://leetcode.com/problems/subrectangle-queries
type SubrectangleQueries struct {
	Rectangle [][]int
}

func NewSubrectangle(rectangle [][]int) SubrectangleQueries {
	return SubrectangleQueries{Rectangle: rectangle}
}

func (sq *SubrectangleQueries) UpdateSubrectangle(row1 int, col1 int, row2 int, col2 int, newValue int) {
	for i := row1; i <= row2; i++ {
		for j := col1; j <= col2; j++ {
			sq.Rectangle[i][j] = newValue
		}
	}
}

func (sq *SubrectangleQueries) GetValue(row int, col int) int {
	return sq.Rectangle[row][col]
}
