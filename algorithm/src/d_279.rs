impl Solution {
    // 效率更高
    pub fn num_squares_s(n: i32) -> i32 {
        let mut dp = vec![0; 1 + n as usize];
        for cap in 1..dp.len() {
            let mut min = i32::MAX;
            let mut j = 1;
            while (j * j) <= cap {
                min = min.min(dp[cap - (j * j) as usize] + 1);
                dp[cap] = min;
                j += 1;
            }
        }
        dp[n as usize]
    }

    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![n + 1; 1 + n as usize];
        dp[0] = 0;
        for cap in 1..dp.len() {
            let mut j = 1;
            while (j * j) <= cap {
                dp[cap] = dp[cap].min(dp[cap - (j * j) as usize] + 1);
                j += 1;
            }
        }
        dp[n as usize]
    }
}

fn square_numbers_up_to_n(n: i32) -> Vec<i32> {
    let mut squares = Vec::new();
    let mut i = 1;

    while i * i <= n {
        squares.push(i * i);
        i += 1;
    }
    squares
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::num_squares(12);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
