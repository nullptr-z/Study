impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut profit = 0;

        for val in prices.into_iter().skip(1) {
            if min > val {
                min = val;
            }
            profit = profit.max(val - min);
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::max_profit(vec![7, 6, 4, 3, 1]);
    }
}

pub struct Solution;
