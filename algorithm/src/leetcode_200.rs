impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut count = 0;
        let mut stack = Vec::new();
        for i in 0..m {
            for j in 0..n {
                let val = grid[i][j];
                if val == '1' {
                    stack.push((i, j));
                    grid[i][j] = '0';
                    count += 1;
                    find_join(&mut grid, &mut stack, m, n);
                }
            }
        }

        count
    }
}

fn find_join(grid: &mut Vec<Vec<char>>, stack: &mut Vec<(usize, usize)>, m: usize, n: usize) {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;

            if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if grid[new_x][new_y] == '1' {
                    stack.push((new_x, new_y));
                    grid[new_x][new_y] = '0'; // Mark as visited
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]);
    }
}

pub struct Solution;
