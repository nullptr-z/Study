impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![0; amount + 1];
        dp[0] = 1;
        for c in coins {
            let c = c as usize;
            for cap in c..dp.len() {
                dp[cap] += dp[cap - c];
            }
        }

        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::change(3, vec![2]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::change(5, vec![1, 2, 5]);

        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
