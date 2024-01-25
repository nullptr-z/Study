impl Solution {
    pub fn complete_backpack(wvs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        for wv in wvs {
            for cap in 1..dp.len() {
                let remain = (cap as i32) - wv[0];
                if remain >= 0 {
                    dp[cap] = dp[cap].max(dp[remain as usize] + wv[1]);
                }
            }
        }

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret =
            Solution::complete_backpack(vec![vec![1, 2], vec![2, 4], vec![3, 4], vec![4, 5]], 5);

        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
