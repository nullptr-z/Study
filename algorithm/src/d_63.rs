impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; obstacle_grid[0].len()];
        for (i, &val) in obstacle_grid[0].iter().enumerate() {
            if val == 1 {
                break;
            }
            dp[i] = 1;
        }

        for rows in obstacle_grid.iter().skip(1) {
            for j in 0..rows.len() {
                if rows[j] == 1 {
                    dp[j] = 0;
                    continue;
                }
                if j != 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::unique_paths_with_obstacles(vec![
            // vec![1, 0],
            // vec![0, 0, 0],
            // vec![0, 1, 0],
            // vec![0, 0, 0],
            vec![1],
        ]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
