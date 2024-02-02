impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        for num in nums {
            if let Err(idx) = dp.binary_search(&num) {
                if idx >= dp.len() {
                    dp.push(num);
                } else {
                    dp[idx] = num;
                }
            }
        }
        dp.len() as i32
    }

    pub fn length_of_lisp(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len()];
        let mut max_len = 1;
        for i in 1..dp.len() {
            dp[i] = 1;
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_len = max_len.max(dp[i]);
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::length_of_lis(vec![0, 3, 1, 6, 2, 2, 7]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
