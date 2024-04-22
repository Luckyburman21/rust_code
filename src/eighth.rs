// Given a binary tree, implement a function that returns the maximum depth of the tree.


use std::cmp;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0,
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            cmp::max(left_depth, right_depth) + 1
        }
    }
}

fn main() {
   
    let mut root = TreeNode::new(3);
    let mut node1 = TreeNode::new(9);
    let mut node2 = TreeNode::new(20);
    let mut node3 = TreeNode::new(15);
    let mut node4 = TreeNode::new(7);

    node2.left = Some(Box::new(node3));
    node2.right = Some(Box::new(node4));
    root.left = Some(Box::new(node1));
    root.right = Some(Box::new(node2));

    let depth = max_depth(Some(Box::new(root)));
    println!("Maximum depth of the tree: {}", depth);
}
