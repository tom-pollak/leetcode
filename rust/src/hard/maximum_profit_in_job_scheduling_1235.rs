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
        start_time: Vec<i32>,
        end_time: Vec<i32>,
        profit: Vec<i32>,
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
        pub fn new(
            start_time: &'a mut Vec<i32>,
            end_time: &'a mut Vec<i32>,
            profit: &'a mut Vec<i32>,
        ) -> Self {
            assert!(
                start_time.len() == end_time.len() && end_time.len() == profit.len(),
                "Parameters must all be of the same length!"
            );

            let jobs = Jobs {
                start_time: start_time.to_vec(),
                end_time: end_time.to_vec(),
                profit: profit.to_vec(),
            };

            // No start_time.is_sorted()
            let mut sorted_start = start_time.clone();
            sorted_start.sort();

            if *start_time == sorted_start {
                jobs
            } else {
                let mut iter: Vec<_> = jobs.into_iter().collect();
                iter.sort_by_key(|k| k.0);
                Jobs::from_iter(iter)
            }
        }

        pub fn get_jobs_from(&'a self, n: usize) -> Jobs {
            Jobs::from_iter(self.into_iter().filter(|i| i.0 >= n as i32))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_iter() {
            let s = &mut vec![1, 2, 3];
            let e = &mut vec![1, 2, 3];
            let p = &mut vec![1, 2, 3];

            let j = Jobs::new(s, e, p);
            let mut iter = j.into_iter();

            assert_eq!(iter.next(), Some((1, 1, 1)));
            assert_eq!(iter.next(), Some((2, 2, 2)));
        }

        #[test]
        fn test_get_jobs_after() {
            let s = &mut vec![1, 2, 3];
            let e = &mut vec![1, 2, 3];
            let p = &mut vec![1, 2, 3];
            let j = Jobs::new(s, e, p);
            assert_eq!(
                j.get_jobs_from(1),
                Jobs::new(&mut vec![1, 2, 3], &mut vec![1, 2, 3], &mut vec![1, 2, 3])
            );
            assert_eq!(
                j.get_jobs_from(2),
                Jobs::new(&mut vec![2, 3], &mut vec![2, 3], &mut vec![2, 3])
            );
            assert_eq!(
                j.get_jobs_from(3),
                Jobs::new(&mut vec![3], &mut vec![3], &mut vec![3])
            );
            assert_eq!(
                j.get_jobs_from(4),
                Jobs::new(&mut vec![], &mut vec![], &mut vec![])
            );
            assert_eq!(
                j.get_jobs_from(5),
                Jobs::new(&mut vec![], &mut vec![], &mut vec![])
            );
        }
    }
}

use std::collections::HashMap;

use jobs::Jobs;

impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(
        mut start_time: Vec<i32>,
        mut end_time: Vec<i32>,
        mut profit: Vec<i32>,
    ) -> i32 {
        let mut best_profit: HashMap<i32, i32> = HashMap::new();
        let jobs = Jobs::new(&mut start_time, &mut end_time, &mut profit);
        Solution::calc_jobs(&jobs, &mut best_profit, 0, 0);
        best_profit
            .to_owned()
            .into_values()
            .into_iter()
            .max()
            .expect("Should be at least one job")
            .to_owned()
    }

    fn calc_jobs(jobs: &Jobs, best_profit: &mut HashMap<i32, i32>, cur_profit: i32, depth: usize) {
        for (_s, e, p) in jobs.into_iter() {
            let new_p = cur_profit + p;
            let x = best_profit.entry(e).or_insert(new_p);
            *x = std::cmp::max(*x, new_p);
            Solution::calc_jobs(
                &jobs.get_jobs_from(e as usize),
                best_profit,
                new_p,
                depth + 1,
            );
        }
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
