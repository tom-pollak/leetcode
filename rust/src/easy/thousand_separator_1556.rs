/*
 * @lc app=leetcode id=1556 lang=rust
 *
 * [1556] Thousand Separator
 */

struct Solution;

// @lc code=start
impl Solution {
    #[allow(dead_code)]
    pub fn thousand_separator(n: i32) -> String {
        let mut string_n = n.to_string();
        if n < 1000 {
        } else if n < 10000 {
            // 9.999
            string_n.insert(1, '.');
        } else if n < 100000 {
            // 99.999
            string_n.insert(2, '.');
        } else {
            // 1234567 -> 7654321 -> 765.4321 -> 1234.567
            string_n = string_n.chars().rev().collect();
            string_n.insert(3, '.');
            string_n = string_n.chars().rev().collect();
        }
        string_n
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::thousand_separator(987), "987");
        assert_eq!(Solution::thousand_separator(1234), "1.234");

        assert_eq!(Solution::thousand_separator(122131), "122.131");
        assert_eq!(Solution::thousand_separator(11122131), "11122.131");
        assert_eq!(Solution::thousand_separator(1), "1");
        assert_eq!(Solution::thousand_separator(12), "12");
        assert_eq!(Solution::thousand_separator(1000), "1.000");
    }
}
