/*
 * @lc app=leetcode id=382 lang=rust
 *
 * [382] Linked List Random Node
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// @lc code=start
// Definition for singly-linked list.
use rand::Rng;

struct Solution {
    nodes: Option<Box<ListNode>>,
    length: i32,
}

#[allow(dead_code)]
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution {
            length: Solution::get_length(&head),
            nodes: head,
        }
    }

    fn get_length(mut head: &Option<Box<ListNode>>) -> i32 {
        let mut length = 0;
        while let Some(n) = head {
            head = &n.next;
            length += 1;
        }
        length
    }

    fn get_random(&self) -> i32 {
        let idx = rand::thread_rng().gen_range(0, self.length);
        let mut node: Option<&ListNode> = self.nodes.as_deref();
        for _ in 0..idx {
            node = node.unwrap().next.as_deref();
        }
        node.unwrap().val
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
// @lc code=end

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works() {}
}

mod old_solution {
    use super::*;

    struct Solution {
        unparsed_nodes: Option<Box<ListNode>>,
        vals: Vec<i32>,
    }

    #[allow(dead_code)]
    impl Solution {
        fn new(head: Option<Box<ListNode>>) -> Self {
            let nodes: Vec<i32> = Vec::new();
            Solution {
                unparsed_nodes: head,
                vals: nodes,
            }
        }

        fn parse_nodes(&mut self, n: usize) {
            while self.vals.len() < n {
                if let Some(node) = &self.unparsed_nodes {
                    let node = node.as_ref().to_owned();
                    self.vals.push(node.val);
                    self.unparsed_nodes = node.next;
                } else {
                    return;
                }
            }
        }

        fn get_random(&self) -> i32 {
            let idx = rand::thread_rng().gen_range::<usize, usize, _>(0, self.vals.len());
            self.vals[idx]
        }
    }
}
