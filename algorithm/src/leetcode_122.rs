impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                sum += prices[i] - prices[i - 1];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
