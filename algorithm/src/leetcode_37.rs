impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn backtracking(board: &mut Vec<Vec<char>>, row: usize) -> bool {
            // 剪枝: row作为起始行,而不是从头开始找'.'
            for row in row..board.len() {
                for col in 0..board[row].len() {
                    if board[row][col] != '.' {
                        continue;
                    }
                    // 枚举1..9数字
                    for value in '1'..='9' {
                        if validate(&board, row, col, value) {
                            board[row][col] = value;
                            // 向后探索
                            if backtracking(board, row) {
                                // 所有空格满足了，开始返回结果
                                return true;
                            }
                            // 后序有不满足的，回溯到这里
                            board[row][col] = '.';
                        }
                    }
                    // 无法找到满足条件项
                    return false;
                }
            }
            // 所有行都处理完了，会走到这里
            true
        }

        backtracking(board, 0);
    }
}

fn validate(board: &Vec<Vec<char>>, row: usize, col: usize, value: char) -> bool {
    // 行列是否有重复
    for i in 0..board.len() {
        if board[row][i] == value {
            return false;
        }
        if board[i][col] == value {
            return false;
        }
    }

    // 宫格是否有重复
    let r = (row / 3) * 3;
    let c = (col / 3) * 3;
    for i in r..r + 3 {
        for j in c..c + 3 {
            if board[i][j] == value {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let mut borad = &mut [
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'].into(),
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'].into(),
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'].into(),
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'].into(),
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'].into(),
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'].into(),
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'].into(),
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'].into(),
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'].into(),
        ]
        .into();
        Solution::solve_sudoku(borad);
        for rows in borad {
            println!("【 rows 】==> {:?}", rows);
        }
    }
}

pub struct Solution;
