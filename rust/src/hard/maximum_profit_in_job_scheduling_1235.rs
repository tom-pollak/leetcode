/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 */

struct Solution {}

// NOTE: Dynamic programming

// @lc code=start

// use itertools::izip;
//
// impl<'a> IntoIterator for Jobs<'a> {
//     type Item = (&'a i32, &'a i32, &'a i32);
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         izip!(
//             self.start_time.into_iter(),
//             self.end_time.into_iter(),
//             self.profit.into_iter()
//         )
//         .collect::<Vec<Self::Item>>()
//         .into_iter()
//     }
// }

mod jobs {
    #[derive(PartialEq, Debug)]
    pub struct Jobs<'a> {
        start_time: &'a [i32],
        end_time: &'a [i32],
        profit: &'a [i32],
    }

    impl<'a> IntoIterator for &'a Jobs<'a> {
        type Item = (&'a i32, &'a i32, &'a i32);
        type IntoIter = JobsIntoIterator<'a>;

        fn into_iter(self) -> Self::IntoIter {
            JobsIntoIterator {
                jobs: self,
                index: 0,
            }
        }
    }

    pub struct JobsIntoIterator<'a> {
        jobs: &'a Jobs<'a>,
        index: usize,
    }

    impl<'a> Iterator for JobsIntoIterator<'a> {
        type Item = (&'a i32, &'a i32, &'a i32);

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
            Some((s, e, p))
        }
    }

    impl<'a> Jobs<'a> {
        pub fn new(start_time: &'a [i32], end_time: &'a [i32], profit: &'a [i32]) -> Self {
            assert!(
                start_time.len() == end_time.len() && end_time.len() == profit.len(),
                "Parameters must all be of the same length!"
            );
            Jobs {
                start_time,
                end_time,
                profit,
            }
        }

        pub fn get_jobs_after(&'a self, n: usize) -> Jobs {
            // As jobs are one-indexed and vec is zero-indexed, we don't need to add 1
            // assert!(n <= self.start_time.len(), "N must be less than vec");
            if n >= self.start_time.len() {
                Jobs::new(&[], &[], &[])
            } else {
                Jobs::new(
                    self.start_time.split_at(n).1,
                    self.end_time.split_at(n).1,
                    self.profit.split_at(n).1,
                )
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_iter() {
            let j = Jobs::new(&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]);
            let mut iter = j.into_iter();

            assert_eq!(iter.next(), Some((&1, &1, &1)));
            assert_eq!(iter.next(), Some((&2, &2, &2)));
        }

        #[test]
        fn test_get_jobs_after() {
            let j = Jobs::new(&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]);
            assert_eq!(j.get_jobs_after(1), Jobs::new(&[2, 3], &[2, 3], &[2, 3]));
            assert_eq!(j.get_jobs_after(2), Jobs::new(&[3], &[3], &[3]));
            assert_eq!(j.get_jobs_after(3), Jobs::new(&[], &[], &[]));
            assert_eq!(j.get_jobs_after(4), Jobs::new(&[], &[], &[]));
        }
    }
}

use jobs::Jobs;

impl Solution {
    #[allow(dead_code)]
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut best_profit: Vec<Option<i32>> = Vec::new();
        let jobs = Jobs::new(&start_time, &end_time, &profit);
        let res = Solution::calc_jobs(jobs, &mut best_profit, 0);
        res.iter()
            .max()
            .expect("Should be at least one job")
            .unwrap()
            .to_owned()
    }

    fn calc_jobs<'a>(
        jobs: Jobs,
        mut best_profit: &'a mut Vec<Option<i32>>,
        cur_profit: i32,
    ) -> &'a mut Vec<Option<i32>> {
        for (s, e, p) in jobs.into_iter() {
            let new_p = cur_profit + p;
            if let Some(x) = best_profit.get_mut(*e as usize) {
                if let Some(y) = x {
                    *y = std::cmp::max(*y, new_p);
                } else {
                    x.replace(new_p);
                }
            } else {
                best_profit.extend(vec![None; *e as usize - best_profit.len() + 1]);
                let n = best_profit
                    .get_mut(*e as usize)
                    .expect("Should be value as just extended");
                assert_eq!(*n, None);
                n.replace(new_p);
            }
            best_profit =
                Solution::calc_jobs(jobs.get_jobs_after(*e as usize - 1), best_profit, new_p);
        }
        println!("{:?}", best_profit);
        best_profit
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
    }
}
