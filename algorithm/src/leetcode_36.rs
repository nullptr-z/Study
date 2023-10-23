//

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut group_row = 0;

        let mut map = HashMap::new(); // 记录3x3的小数数组，也可以使用三维数组
        let mut rows = [[0; 9]; 9]; // 行记录
        let mut columns = [[0; 9]; 9]; // 列记录

        while group_row < 9 {
            let mut grid_r = 0;
            while grid_r < 9 {
                let mut grid = 0;
                while grid < 9 {
                    let row = ((((grid / 3) as f64).floor() as i32) + group_row) as usize; // 其实不用向下取整，默认就是
                    let col = (grid % 3 + grid_r) as usize;

                    grid += 1;
                    if board[row][col] == '.' {
                        continue;
                    }
                    let index = ((board[row][col] as u8) - (b'0' as u8) - 1) as usize;
                    rows[row][index] += 1;
                    columns[col][index] += 1;
                    if columns[col][index] > 1 || rows[row][index] > 1 {
                        return false;
                    }

                    // print!("{},{}({})\t", row, col, board[row][col]);
                    let key = format!("key-{}-{}:{}", group_row, grid_r, board[row][col]);
                    if map.get(&key).is_some() {
                        return false;
                    } else {
                        map.insert(key, board[row][col]);
                    }
                }
                grid_r += 3;
                // print!("\n\n");
            }
            group_row += 3;
        }

        true
    }

    pub fn is_valid_sudokus(board: Vec<Vec<char>>) -> bool {
        let mut row = 0;
        let mut col = 0;
        let mut count = 0;

        for (i, value) in board.iter().enumerate() {
            for (j, val) in value.iter().enumerate() {
                print!("{},{}\t", i, j);
            }
            print!("\n\n");
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let sudoku: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '5', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudoku(sudoku);
        println!("【 result 】==> {:?}", result);
    }
    #[test]
    fn should_works() {
        let sudoku: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = Solution::is_valid_sudokus(sudoku);
    }
}
