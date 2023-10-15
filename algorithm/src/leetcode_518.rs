//

pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; (amount + 1) as usize]; coins.len() + 1];

        for coin in 1..=coins.len() {
            let t_coins = coins[coin - 1] as usize;
            dp[coin][0] = 1; // 上限=面额的情况，此时+1
            for count in t_coins..=(amount as usize) {
                if t_coins > count {
                    dp[coin][count] = dp[coin - 1][count];
                    continue;
                }
                // count - t_coins的含义：距离上一个t_coins的倍数位置，累计值是多少
                dp[coin][count] = dp[coin - 1][count] + dp[coin][count - t_coins];
            }
        }

        // for d in dp.iter() {
        //     println!("【 d 】==> {:?}", d);
        // }

        let count = dp[coins.len()][(amount) as usize];
        // println!("【 count 】==> {:?}", count);

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::change(5, vec![1, 2, 5]);
        let result = Solution::change(3, vec![2]);
        let result = Solution::change(10, vec![5]);
        let result = Solution::change(5, vec![2, 5]);
        println!("【 result 】==> {:?}", result);
    }
}
