use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn backtracking(
            grid: &mut Vec<Vec<i32>>,
            result: &mut Vec<Vec<String>>,
            point: &mut HashSet<usize>,
            row: usize,
        ) {
            for col in 1..grid[row].len() - 1 {
                let row_s = 100 + row;
                let col_s = 200 + col;

                let mut flags = false;
                let mut c = col;
                let mut r = row;
                while c > 0 && r > 0 {
                    c -= 1;
                    r -= 1;
                    if grid[r][c] == 0 {
                        flags = true;
                        break;
                    }
                }
                if flags {
                    continue;
                }

                let mut c = col;
                let mut r = row;
                while c < grid.len() && r > 0 {
                    r -= 1;
                    c += 1;
                    if grid[r][c] == 0 {
                        flags = true;
                        break;
                    }
                }
                if flags {
                    continue;
                }

                if point.get(&row_s).is_none() && point.get(&col_s).is_none() {
                    grid[row][col] = 0;
                    point.insert(row_s.clone());
                    point.insert(col_s.clone());
                    if row == grid.len() - 1 {
                        let mut temp: Vec<String> = vec![];
                        for row in 1..grid.len() {
                            let mut str = vec![];
                            for col in 1..grid[row].len() - 1 {
                                if grid[row][col] == 1 {
                                    str.push(".");
                                } else {
                                    str.push("Q");
                                }
                            }
                            temp.push(str.join(""))
                        }
                        result.push(temp);
                        grid[row][col] = 1;
                        point.remove(&row_s);
                        point.remove(&col_s);
                        return;
                    }
                } else {
                    continue;
                }
                backtracking(grid, result, point, row + 1);
                point.remove(&row_s);
                point.remove(&col_s);
                grid[row][col] = 1;
            }
        }

        let n = (n + 1) as usize;
        let mut grid = vec![vec![1; n + 1]; n];
        let mut point = HashSet::new();
        let mut result = vec![];
        backtracking(&mut grid, &mut result, &mut point, 1);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::solve_n_queens(5);
        for m in ret {
            for s in m {
                println!("{:?}", s);
            }
            println!("--------------------------------")
        }
    }
}

pub struct Solution;
