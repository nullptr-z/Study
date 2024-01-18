impl Solution {
    pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        let mut dp = vec![cost[0], cost[1]];
        cost.push(0); // 追加一个表示顶楼
        for i in 2..cost.len() {
            let val = cost[i] + (dp[i - 1]).min(dp[i - 2]);
            dp.push(val);
        }
        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
