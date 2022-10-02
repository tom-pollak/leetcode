/*
 * @lc app=leetcode id=446 lang=rust
 *
 * [446] Arithmetic Slices II - Subsequence
 */

struct Solution;

// @lc code=start
impl Solution {
    fn make_combinations(nums: &[i32], len: usize) -> Vec<Vec<i32>> {
        if len == 1 {
            return nums.iter().map(|&n| vec![n]).collect();
        }

        let mut combs: Vec<Vec<i32>> = Vec::new();
        for (i, &n) in nums.iter().enumerate() {
            Solution::make_combinations(&nums[i + 1..], len - 1)
                .iter_mut()
                .for_each(|v| {
                    v.insert(0, n);
                    combs.push(v.to_owned())
                });
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
        let perms = Solution::make_combinations(&nums, 3);
        for p in perms {
            if Solution::is_arithmetic_seq(&p) {
                count += 1
            }
        }
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
