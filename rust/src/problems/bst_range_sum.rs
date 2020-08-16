// https://leetcode.com/problems/range-sum-of-bst

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> i32 {
    if let Some(root) = root {
        let mut sum = 0;
        if l <= root.borrow().val && root.borrow().val <= r {
            sum += root.borrow().val;
        }

        if l < root.borrow().val {
            sum += range_sum_bst(root.borrow().left.clone(), l, r);
        }

        if r > root.borrow().val {
            sum += range_sum_bst(root.borrow().right.clone(), l, r);
        }
        return sum;
    }
    return 0;
}