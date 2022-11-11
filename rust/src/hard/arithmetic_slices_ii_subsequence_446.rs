/*
 * @lc app=leetcode id=446 lang=rust
 *
 * [446] Arithmetic Slices II - Subsequence
 */

use std::collections::HashSet;

struct Solution;

// New question plx
// @lc code=start
impl Solution {
    fn make_combinations(nums: Vec<i32>, min_len: usize) -> HashSet<Vec<i32>> {
        // let mut combs: Vec<Vec<i32>> = Vec::new();
        let mut combs: HashSet<Vec<i32>> = HashSet::new();
        if nums.len() >= min_len {
            for (i, &_n) in nums.iter().enumerate() {
                let mut new_nums = nums.clone();
                new_nums.remove(i);
                combs.extend(Solution::make_combinations(new_nums, min_len));
            }
            combs.insert(nums);
        }
        combs
    }

    fn is_arithmetic_seq(seq: &[i32]) -> bool {
        let mut diff = None;
        let mut prev = None;
        for &n in seq {
            match prev {
                None => {}
                Some(p) => match diff {
                    None => diff = Some(n - p),
                    Some(d) => {
                        if n - p != d {
                            return false;
                        }
                    }
                },
            };
            prev = Some(n);
        }
        true
    }

    #[allow(dead_code)]
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let combs = Solution::make_combinations(nums, 3);
        let mut arithm_combs = Vec::new();
        for p in combs {
            if Solution::is_arithmetic_seq(&p) {
                count += 1;
                arithm_combs.push(p);
            }
        }
        println!("{:?}", arithm_combs);
        count
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );

        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 1]), 0);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2]), 0);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3]), 1);
    }
}
