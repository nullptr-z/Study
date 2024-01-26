impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for &c in coins.iter() {
            let c = c as usize;
            for cap in c..dp.len() {
                dp[cap] = dp[cap].min(dp[cap - c] + 1)
            }
        }
        if dp[amount as usize] > amount {
            return -1;
        }
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::coin_change(vec![2], 3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
