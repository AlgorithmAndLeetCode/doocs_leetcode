// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, res: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.as_ref().unwrap().as_ref().borrow();
        let left = Self::dfs(&root.left, root.val, res);
        let right = Self::dfs(&root.right, root.val, res);
        *res = (*res).max(left + right);
        if root.val == target {
            return left.max(right) + 1;
        }
        0
    }

    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut res = 0;
        Self::dfs(
            &root,
            root.as_ref().unwrap().as_ref().borrow().val,
            &mut res,
        );
        res
    }
}
