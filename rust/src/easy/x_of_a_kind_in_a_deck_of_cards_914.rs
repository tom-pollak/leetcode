/*
 * @lc app=leetcode id=914 lang=rust
 *
 * [914] X of a Kind in a Deck of Cards
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut cards: HashMap<i32, i32> = HashMap::new();
        for card in deck {
            cards.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        if cards.len() == 1 {
            return cards.values().next().unwrap() >= &2;
        }

        let mut cur_gcd = None;
        cards
            .into_values()
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|w| {
                let gcd = Solution::gcd(w[0], w[1]);
                if cur_gcd.is_none() {
                    cur_gcd = Some(gcd);
                } else {
                    cur_gcd = Some(Solution::gcd(cur_gcd.unwrap(), gcd));
                }
            });
        match cur_gcd {
            Some(v) => v >= 2,
            None => false,
        }
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert!(Solution::has_groups_size_x(vec![1, 2, 3, 4, 4, 3, 2, 1]));
        assert!(!Solution::has_groups_size_x(vec![1, 1, 1, 2, 2, 2, 3, 3]));
        assert!(Solution::has_groups_size_x(vec![1, 1, 2, 2, 2, 2]));
        assert!(Solution::has_groups_size_x(vec![
            1, 1, 1, 1, 2, 2, 2, 2, 2, 2
        ]));
        assert!(!Solution::has_groups_size_x(vec![]));
        assert!(!Solution::has_groups_size_x(vec![1]));
        assert!(Solution::has_groups_size_x(vec![1, 1]));
        assert!(!Solution::has_groups_size_x(vec![1, 2]));
    }
}
