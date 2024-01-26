//

pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![(amount + 1); (amount + 1) as usize];
        dp[0] = 0;

        for count in 1..=(amount as usize) {
            for coin in coins.iter() {
                let coin = *coin as usize;
                if coin <= count {
                    dp[count] = dp[count].min(dp[count - (coin as usize)] + 1);
                }
            }
        }

        println!("【 dp 】==> {:?}", dp);
        let min = dp[amount as usize];

        if min > amount {
            -1
        } else {
            min
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let resule = Solution::coin_change(vec![1, 2, 5], 11);
        // assert_eq!(resule, 3);

        // let resule = Solution::coin_change(vec![2], 3);

        // let resule = Solution::coin_change(vec![186, 419, 83, 408], 6249);
        println!("【 resule 】==> {:?}", resule);
    }
}
