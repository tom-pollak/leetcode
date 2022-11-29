/*
 * @lc app=leetcode id=2058 lang=rust
 *
 * [2058] Find the Minimum and Maximum Number of Nodes Between Critical Points
 */

// Definition for singly-linked list.
use std::ops::DerefMut;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

// @lc code=start
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        vec![1]
    }
}
// @lc code=end

fn setup_list_node(mut values: Vec<i32>) -> Option<Box<ListNode>> {
    if values.is_empty() {
        return None;
    }
    let box_list_node = |v| Box::new(ListNode::new(v));

    let mut tail = box_list_node(values.remove(0));

    for &val in values.iter() {
        let node = box_list_node(val);
        tail.deref_mut().next = Some(node);
        tail = node;
    }
    Some(tail)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn list_node() {
        assert_eq!(
            setup_list_node(vec![1, 2, 3]),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        )
    }

    #[test]
    fn nodes_crit() {
        assert_eq!(Solution::nodes_between_critical_points(None), vec![-1, -1]);
        assert_eq!(
            Solution::nodes_between_critical_points(Some(Box::new(ListNode::new(1)))),
            vec![-1, -1]
        );
    }
}
