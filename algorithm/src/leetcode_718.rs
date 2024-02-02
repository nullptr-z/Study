impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let m = nums1.len() + 1;
        let n = nums2.len() + 1;
        let mut dp = vec![vec![0; n]; m];
        for i in 1..m {
            for j in 1..n {
                if nums1[i - 1] == nums2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    max_len = max_len.max(dp[i][j]);
                }
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
        let ret = Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
