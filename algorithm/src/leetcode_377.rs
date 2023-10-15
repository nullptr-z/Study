//

pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; (target + 1) as usize];
        dp[0] = 1;

        for i in 1..=(target as usize) {
            for num in nums.iter() {
                let num = *num as usize;
                if num <= i {
                    dp[i] += dp[i - num];
                }
            }
        }

        println!("【 dp 】==> {:?}", dp);

        *dp.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::combination_sum4(vec![1, 2, 3], 4);
    }
}
