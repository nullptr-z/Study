//

pub struct Solution;

impl Solution {
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        // nums.sort_by(|a, b| a.cmp(b));
        let sum = {
            let mut sum = 0;
            for num in nums.iter() {
                sum += *num;
            }
            sum
        };
        if sum % 2 == 1 {
            return false;
        }
        let count = nums.len();
        let square = (sum >> 1) as usize;

        let mut dp = vec![vec![0; square + 1]; count + 1];
        for r in 1..=count {
            for l in 1..=square {
                let val = nums[r - 1];
                if val <= (l as i32) {
                    dp[r][l] = dp[r - 1][l].max(dp[r - 1][((l as i32) - val) as usize] + val);
                } else {
                    dp[r][l] = dp[r - 1][l];
                }
                if dp[r][l] == (square as i32) {
                    return true;
                }
            }
        }
        // println!("【 nums 】==> {:?}", nums);
        // for item in dp.iter() {
        //     println!("【 item 】==> {:?}", item);
        // }

        let result = dp.last().unwrap().last().unwrap();

        *result == (square as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn can_partition() {
        let result = Solution::can_partition(vec![1, 2, 3, 5]);
        assert_eq!(result, false);

        let result = Solution::can_partition(vec![1, 2, 8, 5]);
        assert_eq!(result, true);

        let result = Solution::can_partition(vec![23, 13, 11, 7, 6, 5, 5]);
        assert_eq!(result, true);
    }
}
