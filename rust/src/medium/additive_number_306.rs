/*
 * @lc app=leetcode id=306 lang=rust
 *
 * [306] Additive Number
 */

struct Solution;

// @lc code=start
impl Solution {
    fn recursive_seq(nums: Vec<u32>) -> u32 {
        if nums.is_empty() {
            return 0;
        };
        let first_digit = nums.remove(0);
    }

    pub fn is_additive_number(num: String) -> bool {
        let nums: Vec<u32> = num
            .chars()
            .map(|c| c.to_digit(10).expect("All characters should be a number"))
            .collect();
        Solution::recursive_seq(nums);
        false
    }
}
// @lc code=end
