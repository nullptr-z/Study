impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>();
        let pack = (sum / 2) as usize;
        let mut dp = vec![0; pack + 1];
        for i in stones {
            for cap in ((i as usize)..dp.len()).rev() {
                dp[cap] = dp[cap].max(dp[cap - i as usize] + i)
            }
        }

        (sum - dp[pack]) - dp[pack]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]);
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::last_stone_weight_ii(vec![
        //     1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 14, 23, 37, 61, 98,
        // ]);
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
