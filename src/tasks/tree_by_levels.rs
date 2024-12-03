// https://www.codewars.com/kata/52bef5e3588c56132c0003bc/train/rust

use std::ops::Deref;

struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
  }


  pub fn tree_by_levels(root: &Node) -> Vec<u32> {
    fn flat_tree(node: &Node) -> Vec<u32> {
        let mut result = Vec::new();
        let mut queue = vec![node];

        while !queue.is_empty() {
            let current_node = queue.remove(0);
            result.push(current_node.value);

            if let Some(ref left) = current_node.left {
                queue.push(left);
            }
            if let Some(ref right) = current_node.right {
                queue.push(right);
            }
        }

        result
    }

    flat_tree(root)
}