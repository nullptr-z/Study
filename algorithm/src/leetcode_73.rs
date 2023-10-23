//

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut rows = vec![false; m];
        let mut column = vec![false; n];

        let mut i = 0;
        while i < m {
            let mut j = 0;
            while j < n {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    column[j] = true;
                }

                j += 1;
            }
            i += 1;
        }

        let mut i = 0;
        while i < m {
            let mut j = 0;
            let get_row = rows[i];
            while j < n {
                if get_row {
                    matrix[i][j] = 0;
                } else {
                    let get_col = column[j];
                    if get_col {
                        matrix[i][j] = 0;
                    }
                }

                j += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let mut arr = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .to_vec()
            .into_iter()
            .map(|item| item.into())
            .collect();

        Solution::set_zeroes(&mut arr);
    }
}
