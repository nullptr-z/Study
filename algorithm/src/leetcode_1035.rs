impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums2.len(), nums1.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                if nums1[j - 1] == nums2[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1; // 对角线
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]); // 左边和上方
                }
            }
        }

        for d in &dp {
            println!("【 d 】==> {:?}", d);
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::longest_common_subsequencess("bsbininm".into(), "jmjkbkjkv".into());
        // println!("【 ret 】==> {:?} {}", ret, 1);
        // let ret = Solution::longest_common_subsequence("oxcpqrsvwf".into(), "shmtulqrypy".into());
        // println!("【 ret 】==> {:?} {}", ret, 2);
        let ret = Solution::max_uncrossed_lines(
            vec![2, 5, 1, 2, 5].into(),
            vec![10, 5, 2, 1, 5, 2].into(),
        );
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::longest_common_subsequence("ezupkr".into(), "ubmrapg".into());
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
