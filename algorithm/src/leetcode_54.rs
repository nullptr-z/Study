//

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let len = m * n;

        let mut row_range = (0, m - 1);
        let mut col_range = (0, n - 1);

        let mut result = Vec::with_capacity(matrix.len());
        let mut arrow = Arrow::Right;

        let (mut row_ptr, mut col_ptr) = (0, 0);

        while result.len() < len {
            match arrow {
                Arrow::Right => {
                    while col_ptr <= col_range.1 {
                        result.push(matrix[row_ptr][col_ptr]);
                        col_ptr += 1;
                    }
                    col_ptr -= 1;

                    row_ptr += 1;
                    row_range.0 += 1;
                    arrow = Arrow::Down;
                }
                Arrow::Down => {
                    while row_ptr <= row_range.1 {
                        result.push(matrix[row_ptr][col_ptr]);
                        row_ptr += 1;
                    }
                    row_ptr -= 1;

                    if col_range.1 > 0 {
                        col_range.1 -= 1;
                    }
                    arrow = Arrow::Left;
                }
                Arrow::Left => {
                    while (col_ptr as i32 - 1) >= col_range.0 as i32 {
                        col_ptr -= 1;
                        result.push(matrix[row_ptr][col_ptr]);
                    }

                    if row_range.1 > 0 {
                        row_range.1 -= 1;
                    }
                    arrow = Arrow::Up;
                }
                Arrow::Up => {
                    while (row_ptr as i32 - 1) >= row_range.0 as i32 {
                        row_ptr -= 1;
                        result.push(matrix[row_ptr][col_ptr]);
                    }

                    col_ptr += 1;
                    col_range.0 += 1;
                    arrow = Arrow::Right;
                }
            }
        }
        // println!("【 result 】==> {:?}", result);

        result
    }
}

enum Arrow {
    Left,
    Right,
    Down,
    Up,
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let result = Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        let result = Solution::spiral_order(vec![vec![7], vec![8], vec![9]]);
        println!("【 result 】==> {:?}", result);

        // assert_eq!(result, vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }
}
