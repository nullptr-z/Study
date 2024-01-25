impl Solution {
    pub fn complete_backpack(wvs: Vec<Vec<usize>>, n: usize) -> i32 {
        let mut dp = vec![0; n + 1];
        for wv in wvs {
            for cap in wv[0]..dp.len() {
                dp[cap] = dp[cap].max(dp[cap - wv[0]] + wv[1]);
            }
        }

        *dp.last().unwrap() as i32
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
