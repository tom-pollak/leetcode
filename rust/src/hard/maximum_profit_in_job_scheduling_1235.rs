/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 */

struct Solution;

// NOTE: Dynamic programming

// @lc code=start

mod jobs {
    use std::iter::FromIterator;

    #[derive(PartialEq, Eq, Debug)]
    pub struct Jobs {
        pub start_time: Vec<i32>,
        pub end_time: Vec<i32>,
        pub profit: Vec<i32>,
    }

    impl<'a> IntoIterator for &'a Jobs {
        type Item = (i32, i32, i32);
        type IntoIter = JobsIntoIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            JobsIntoIterator {
                jobs: self,
                index: 0,
            }
        }
    }

    pub struct JobsIntoIterator<'a> {
        jobs: &'a Jobs,
        index: usize,
    }

    impl<'a> Iterator for JobsIntoIterator<'a> {
        type Item = (i32, i32, i32);

        fn next(&mut self) -> Option<Self::Item> {
            let s = self.jobs.start_time.get(self.index)?;
            let e = self
                .jobs
                .end_time
                .get(self.index)
                .expect("If the first one unwraps, they all should");
            let p = self
                .jobs
                .profit
                .get(self.index)
                .expect("If the first one unwraps, they all should");
            self.index += 1;
            Some((*s, *e, *p))
        }
    }

    // use itertools::Itertools; // Not in leetcode
    trait FindPosition: Iterator {
        fn find_position<P>(&mut self, mut pred: P) -> Option<(usize, Self::Item)>
        where
            P: FnMut(&Self::Item) -> bool,
        {
            for (index, elt) in self.enumerate() {
                if pred(&elt) {
                    return Some((index, elt));
                }
            }
            None
        }
    }

    impl FromIterator<(i32, i32, i32)> for Jobs {
        fn from_iter<T: IntoIterator<Item = (i32, i32, i32)>>(iter: T) -> Jobs {
            let mut start_time = Vec::new();
            let mut end_time = Vec::new();
            let mut profit = Vec::new();

            for (s, e, p) in iter {
                start_time.push(s);
                end_time.push(e);
                profit.push(p);
            }

            Jobs {
                start_time,
                end_time,
                profit,
            }
        }
    }

    impl<'a> FindPosition for JobsIntoIterator<'a> {}

    impl<'a> ExactSizeIterator for JobsIntoIterator<'a> {
        fn len(&self) -> usize {
            self.jobs.start_time.len()
        }
    }

    impl<'a> Jobs {
        pub fn new() -> Self {
            Jobs {
                start_time: Vec::new(),
                end_time: Vec::new(),
                profit: Vec::new(),
            }
        }

        pub fn get_jobs_from(&'a self, n: i32) -> Jobs {
            Jobs::from_iter(self.into_iter().filter(|i| i.0 >= n))
        }
    }

    impl Default for Jobs {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_iter() {
            let j = Jobs {
                start_time: vec![1, 2, 3],
                end_time: vec![1, 2, 3],
                profit: vec![1, 2, 3],
            };
            let mut iter = j.into_iter();

            assert_eq!(iter.next(), Some((1, 1, 1)));
            assert_eq!(iter.next(), Some((2, 2, 2)));
        }

        #[test]
        fn test_get_jobs_after() {
            let j = Jobs {
                start_time: vec![1, 2, 3],
                end_time: vec![1, 2, 3],
                profit: vec![1, 2, 3],
            };

            assert_eq!(j.get_jobs_from(1), j);
            assert_eq!(
                j.get_jobs_from(2),
                Jobs {
                    start_time: vec![2, 3],
                    end_time: vec![2, 3],
                    profit: vec![2, 3],
                }
            );
            assert_eq!(
                j.get_jobs_from(3),
                Jobs {
                    start_time: vec![3],
                    end_time: vec![3],
                    profit: vec![3],
                }
            );
            assert_eq!(j.get_jobs_from(4), Jobs::new());
            assert_eq!(j.get_jobs_from(5), Jobs::new());
        }
    }
}

use std::collections::HashMap;

use jobs::Jobs;

impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        assert!(
            start_time.len() == end_time.len() && end_time.len() == profit.len(),
            "Parameters must all be of the same length!"
        );

        let jobs_is_sorted = Solution::is_sorted(&start_time);

        let mut jobs = Jobs {
            start_time,
            end_time,
            profit,
        };

        if !jobs_is_sorted {
            let mut iter: Vec<_> = jobs.into_iter().collect();
            iter.sort_by_key(|k| k.0);
            jobs = Jobs::from_iter(iter);
        }

        let mut best_profit: HashMap<i32, i32> = HashMap::new();
        Solution::calc_jobs(&jobs, &mut best_profit, 0);
        best_profit
            .into_values()
            .max()
            .expect("Should be at least one job")
    }

    fn calc_jobs(jobs: &Jobs, best_profit: &mut HashMap<i32, i32>, cur_profit: i32) {
        for (_s, e, p) in jobs.into_iter() {
            let new_p = cur_profit + p;
            let is_best = !best_profit.iter().any(|(&k, &v)| k <= e && v > new_p);
            if is_best {
                best_profit
                    .entry(e)
                    .and_modify(|entry| *entry = new_p)
                    .or_insert(new_p);
                Solution::calc_jobs(&jobs.get_jobs_from(e), best_profit, new_p);
            }
        }
    }

    fn is_sorted<T: Ord>(list: &[T]) -> bool {
        if list.is_empty() {
            return true;
        }
        let prev = &list[0];
        for i in list.iter() {
            if i < prev {
                return false;
            }
        }
        true
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]),
            120
        );

        // 1 -> 3,  4 -> 6,  6 -> 9,
        // It can't pick up the 6 at the same time
        // [None, None, None, 20, None, 90, None, None, 150]
        assert_eq!(
            Solution::job_scheduling(
                vec![1, 2, 3, 4, 6],
                vec![3, 5, 10, 6, 9],
                vec![20, 20, 100, 70, 60]
            ),
            150
        );

        assert_eq!(
            Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]),
            6
        );

        assert_eq!(
            Solution::job_scheduling(
                vec![4, 2, 4, 8, 2],
                vec![5, 5, 5, 10, 8],
                vec![1, 2, 8, 10, 4]
            ),
            18
        );
    }
}
