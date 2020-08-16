package problems

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// https://leetcode.com/problems/range-sum-of-bst
func RangeSumBST(root *TreeNode, L int, R int) int {
	if root == nil {
		return 0
	}
	if root.Val >= L && root.Val <= R {
		return root.Val + RangeSumBST(root.Left, L, R) + RangeSumBST(root.Right, L, R)
	}
	return RangeSumBST(root.Left, L, R) + RangeSumBST(root.Right, L, R)
}
