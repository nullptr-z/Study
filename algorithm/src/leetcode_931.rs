//

pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(triangle: Vec<Vec<i32>>) -> i32 {
        let mut len = triangle.len();
        let col = triangle[0].len();
        let mut dp = triangle.last().unwrap().to_owned();
        let mut dps = triangle.last().unwrap().to_owned();
        len -= 1;

        while len > 0 {
            len -= 1;
            for (i, val) in triangle[len].iter().enumerate() {
                let mut temp = dp[i] + val;
                if i > 0 {
                    temp = temp.min(dp[i - 1] + val);
                }
                if i < (col - 1) {
                    temp = temp.min(dp[i + 1] + val);
                }
                dps[i] = temp;
            }
            dp = dps.clone();
        }

        let mut min = i32::MAX;
        for item in dp.iter() {
            min = min.min(*item);
        }

        // println!("【 dp 】==> {:?}", min);
        min
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]);
    }
}
