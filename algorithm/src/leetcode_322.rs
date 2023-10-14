//

pub struct Solution;

impl Solution {
    pub fn coin_changes(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut dp = vec![vec![0; (amount + 1) as usize]; coins.len() + 1];
        let mut ct = dp.clone();
        let mut min = amount as usize;

        for coin_idx in 1..=coins.len() {
            let coin = coins[coin_idx - 1] as usize;
            for limit in 1..=(amount as usize) {
                if coin <= limit {
                    // dp[coin_idx][limit] = dp[coin_idx][limit - coin] + coin;
                    ct[coin_idx][limit] = ct[coin_idx][limit - coin] + 1;
                } else {
                    // dp[coin_idx][limit] = dp[coin_idx - 1][limit];
                    ct[coin_idx][limit] = ct[coin_idx - 1][limit];
                }
            }
            min = min.min(*ct[coin_idx].last().unwrap());
            println!("【 min 】==> {:?}", min);
        }

        for i in dp.iter() {
            println!("【dp：{:?}", i);
        }
        println!("---------------------",);
        for i in ct.iter() {
            println!("【ct：{:?}", i);
        }
        let count = dp.last().unwrap().last().unwrap();

        if (*count as i32) == 0 {
            return -1;
        }

        if (*count as i32) < amount {
            // return -1;
            return (*count as i32) - amount;
        }

        min as i32
    }

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
