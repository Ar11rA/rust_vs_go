use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::Borrow;

// https://leetcode.com/problems/path-sum/
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
  #[inline]
  fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
  let mut is_path_sum_possible: bool = false;
  dfs(root, sum, 0, &mut is_path_sum_possible);
  return is_path_sum_possible;
}

fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, mut acc: i32, is_path_sum_possible: &mut bool) {
  if let Some(v) = root {
    let g: &RefCell<TreeNode> = v.borrow();
    acc += g.clone().into_inner().val;
    if acc == sum {
      *is_path_sum_possible = true;
    }
    if g.clone().into_inner().left.is_some() {
      dfs(g.clone().into_inner().left.clone(), sum, acc, is_path_sum_possible);
    }
    if g.clone().into_inner().right.is_some() {
      dfs(g.clone().into_inner().right.clone(), sum, acc, is_path_sum_possible);
    }
  }
}