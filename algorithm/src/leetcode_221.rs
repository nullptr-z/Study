//

pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut max_level: usize = 0;
        let mut row = 0;
        while row < m {
            let mut col = 0;
            while col < n {
                let val = matrix[row][col];
                if val == '1' {
                    let mut level = 1;
                    max_level = max_level.max(level);
                    let mut flag = true;
                    while flag && (level + row) < m && (level + col) < n {
                        let r = row + level;
                        let l = col + level;

                        for i in 0..=level {
                            flag = matrix[r][col + i] == '1';
                            if !flag {
                                break;
                            }
                            flag = matrix[row + i][l] == '1';
                            if !flag {
                                break;
                            }
                        }
                        if !flag {
                            break;
                        }
                        level += 1;
                    }
                    max_level = max_level.max(level);
                }
                col += 1;
                // col += max_level.max(1);
                if max_level >= n - col {
                    break;
                }
            }
            row += 1;
            // row += max_level.max(1);
            if max_level >= m - row {
                break;
            }
        }

        println!("【 max_level 】==> {:?}", max_level);
        if max_level == 0 {
            return 0;
        }

        (max_level * max_level) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::maximal_square(vec![
            // vec!['1', '1'],
            // vec!['1', '1'],

            // vec!['1', '0', '1', '0', '0', '0'],
            // vec!['1', '0', '1', '1', '1', '1'],
            // vec!['1', '0', '1', '1', '1', '1'],
            // vec!['1', '1', '1', '0', '0', '0'],
            // vec!['1', '0', '1', '0', '0'],
            // vec!['1', '0', '1', '1', '1'],
            // vec!['1', '1', '1', '1', '1'],
            // vec!['1', '0', '0', '1', '0'],
            // vec!['1', '1', '1', '1', '1', '1', '1', '1'],
            // vec!['1', '1', '1', '1', '1', '1', '1', '0'],
            // vec!['1', '1', '1', '1', '1', '1', '1', '0'],
            // vec!['1', '1', '1', '1', '1', '0', '0', '0'],
            // vec!['0', '1', '1', '1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '1', '0', '1', '1', '1'],
            vec!['0', '1', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '0', '1'],
            vec!['0', '0', '0', '1', '0', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0', '0', '1', '0'],
            vec!['1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '0', '0', '1', '1', '0', '0', '1'],
            vec!['0', '1', '0', '0', '1', '1', '0', '0'],
            vec!['1', '0', '0', '1', '0', '0', '0', '0'],
            // vec!['1', '1', '1', '1', '0'],
            // vec!['1', '1', '1', '1', '0'],
            // vec!['1', '1', '1', '1', '1'],
            // vec!['1', '1', '1', '1', '1'],
            // vec!['0', '0', '1', '1', '1'],
        ]);
    }
}
