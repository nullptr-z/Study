impl Solution {
    pub fn climb_stairs(n: usize, m: usize) -> usize {
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for cap in 1..dp.len() {
            for w in 1..=m {
                if cap >= w {
                    dp[cap] += dp[cap - w];
                }
            }
        }
        dp[n]
    }

    pub fn climb_stairs_s(n: usize, m: usize) -> usize {
        let mut dp = vec![0; m + 1];
        dp[0] = 1;
        for cap in 1..=n {
            for w in 1..=m {
                if cap >= w {
                    dp[w] += dp[cap - w];
                }
            }
            println!("【 dp 】==> {:?}", dp);
        }
        println!("【 dp 】==> {:?}", dp);
        dp[m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::climb_stairs(3, 2);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
