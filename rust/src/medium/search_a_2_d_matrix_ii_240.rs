/*
 * @lc app=leetcode id=240 lang=rust
 *
 * [240] Search a 2D Matrix II
 */

struct Solution;

// Rows sorted left to right
// Columns sorted top to bottom
// Double binary search? start at (m//2, n//2) and search in both directions

// @lc code=start

use std::{cmp, vec};

type Matrix = Vec<Vec<i32>>;

#[allow(dead_code)]
impl Solution {
    fn binary_search_index(nums: &[i32], target: i32) -> usize {
        assert!(!nums.is_empty());
        let mut low = 0;
        let mut high = nums.len() - 1;
        while low < high {
            let mid = (high + low) / 2;
            match target.cmp(&nums[mid]) {
                cmp::Ordering::Equal => return mid,
                cmp::Ordering::Greater => low = mid + 1,
                cmp::Ordering::Less => high = mid,
            }
        }
        high
    }

    // Only for square matrices
    fn construct_diag_axis(matrix: &Matrix) -> Vec<i32> {
        assert_eq!(matrix.len(), matrix[0].len());
        let mut diag_axis = Vec::new();
        (0..matrix.len()).for_each(|i| {
            diag_axis.push(matrix[i][i]);
        });
        diag_axis
    }

    // Returns a new transposed matrix
    fn transpose_2d_mat(matrix: &Matrix) -> Matrix {
        assert!(!matrix.is_empty());
        (0..matrix[0].len())
            .map(|i| matrix.iter().map(|row| row[i]).collect())
            .collect()
    }

    // mxn matrix -> Vector of square matrices
    fn create_square_matrices(mut uneven_mat: Matrix) -> Vec<Matrix> {
        assert!(!uneven_mat.is_empty());
        match uneven_mat.len().cmp(&uneven_mat[0].len()) {
            cmp::Ordering::Equal => return vec![uneven_mat],
            cmp::Ordering::Less => {
                uneven_mat = Solution::transpose_2d_mat(&uneven_mat);
            }
            cmp::Ordering::Greater => {}
        };
        let big_dim = uneven_mat.len();
        let small_dim = uneven_mat[0].len();
        let square_matrix = (0..small_dim)
            .map(|i| uneven_mat[i][0..small_dim].to_vec())
            .collect();
        let mut square_matrices = vec![square_matrix];
        square_matrices.extend(Solution::create_square_matrices(
            (small_dim..big_dim)
                .map(|i| uneven_mat[i].to_vec())
                .collect(),
        ));
        square_matrices
    }

    fn create_mat_block(mat: &Matrix, i: usize) -> Matrix {
        mat[..=i].iter().map(|row| row[i..].to_vec()).collect()
    }

    // O O X X X
    // O O X X X
    // X X X O O
    // X X O O O
    // X X O O O

    pub fn search_matrix(matrix: Matrix, target: i32) -> bool {
        assert!(!matrix.is_empty());
        let square_mats = Solution::create_square_matrices(matrix);
        for mat in square_mats {
            let diag_axis = Solution::construct_diag_axis(&mat);
            let row_i = Solution::binary_search_index(&diag_axis, target);
            if mat[row_i][row_i] == target {
                return true;
            }

            let mat1 = Solution::create_mat_block(&mat, row_i);
            let mat2 = Solution::create_mat_block(&Solution::transpose_2d_mat(&mat), row_i);

            if mat1.len() > 1 && Solution::search_matrix(mat1, target)
                || mat2.len() > 1 && Solution::search_matrix(mat2, target)
            {
                return true;
            }
        }
        false
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(Solution::binary_search_index(&[1, 2, 3, 4, 5], 3), 2);
        assert_eq!(Solution::binary_search_index(&[2], 6), 0);
        assert_eq!(Solution::binary_search_index(&[2], 2), 0);
    }

    #[test]
    fn test_create_mat_block() {
        // 1 2 3
        // 4 5 6
        // 7 8 9

        // 
        let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            Solution::create_mat_block(&mat, 1),
            vec![vec![2, 3], vec![5, 6]]
        );
        assert_eq!(
            Solution::create_mat_block(&Solution::transpose_2d_mat(&mat), 1),
            vec![vec![4, 7], vec![5, 8]]
        );
    }

    #[test]
    fn test_transpose() {
        let uneven_mat = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let transposed_uneven_mat = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(
            Solution::transpose_2d_mat(&uneven_mat),
            transposed_uneven_mat
        );

        assert_eq!(Solution::transpose_2d_mat(&vec![vec![1]]), vec![vec![1]])
    }

    #[test]
    fn test_create_square_mat_from_uneven() {
        // 7x5
        let uneven_mat = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
            vec![28, 29, 32, 36, 48],
            vec![38, 51, 53, 56, 60],
        ];
        let square_mats = Solution::create_square_matrices(uneven_mat);
        assert_eq!(
            square_mats,
            vec![
                vec![
                    vec![1, 4, 7, 11, 15],
                    vec![2, 5, 8, 12, 19],
                    vec![3, 6, 9, 16, 22],
                    vec![10, 13, 14, 17, 24],
                    vec![18, 21, 23, 26, 30],
                ],
                vec![vec![28, 38], vec![29, 51]],
                vec![vec![32, 53], vec![36, 56]],
                vec![vec![48]],
                vec![vec![60]]
            ]
        )
    }

    #[test]
    fn it_works() {
        assert!(Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            5
        ));
        assert!(!Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30]
            ],
            20
        ));
        assert!(Solution::search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
                vec![28, 29, 32, 36, 48],
                vec![38, 51, 53, 56, 60],
            ],
            48
        ));

        assert!(!Solution::search_matrix(vec![vec![5]], 1));
        assert!(Solution::search_matrix(vec![vec![1, 4], vec![2, 5]], 1));
        assert!(Solution::search_matrix(
            vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25]
            ],
            15
        ))
    }
}
