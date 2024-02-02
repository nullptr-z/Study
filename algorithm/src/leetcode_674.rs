impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        let mut max_len = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                dp[i] = dp[i - 1] + 1;
                max_len = max_len.max(dp[i])
            }
        }

        max_len
    }

    pub fn find_length_of_lci(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                count += 1;
                max_len = count.max(max_len);
            } else {
                count = 1
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
