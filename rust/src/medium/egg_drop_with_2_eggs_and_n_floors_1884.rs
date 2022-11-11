use std::{cmp::max_by_key, i32::MAX};

/*
 * @lc app=leetcode id=1884 lang=rust
 *
 * [1884] Egg Drop With 2 Eggs and N Floors
 */

struct Solution;

// @lc code=start
// We are looking to minimize binary search tree height
// Success -> Get another binary search
// What is the height of a binary tree with s = 0.5 (the split)
// 100 -> 50 -> 25 -> 13 -> 7 -> 4 -> 2 -> 1
// = Height: 8
// Worst case (linear): 100 -> 50 -> Linear search (1..48) = 50
// Breaks on first step so we can always guarentee the worst case is the first step
// then the numbers up to 2 before the break
// (we know it breaks on 50, and if it does not break on 48 we know it must break on 49)
//
// let s = 0.25
// 25 -> 44 -> 61 -> 71 -> 79 -> 85 -> 89 -> 92 -> 94 -> 96 -> 97 -> 98 -> 99 -> 100 -- This is rounding up everytime
// 25, 19, 15,  -- decreasing more than the number of iterations -- do we need to adjust our s
// respectively??
// = Height: 14 worst case
// Linear search: 100 -> 25 (1..23)
//
// let s = 0.1
// 10 ->
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        for i in 1..n {
            let deeper_layer = 1 + Solution::two_egg_drop((n - i).div_floor(2));
            if deeper_layer < n {
                return deeper_layer
            }
            // // if 1 + Solution::two_egg_drop(i32::max(i, n - i).div_floor(2)),
            // min_drop = i32::min(
            //     min_drop,
            //     1 + Solution::two_egg_drop(i32::max(i, n - i).div_floor(2)),
            // );
        }
        n
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::two_egg_drop(2), 2);
        assert_eq!(Solution::two_egg_drop(100), 14);
    }
}
