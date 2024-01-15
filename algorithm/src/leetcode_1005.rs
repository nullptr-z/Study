impl Solution {
    pub fn largest_sum_after_k_negations(mut nums: Vec<i32>, mut k: i32) -> i32 {
        // 按绝对值大到小排序
        nums.sort_by_key(|b| std::cmp::Reverse(b.abs()));
        // 将前K个最大负数取反
        for i in 0..nums.len() {
            if k <= 0 {
                break;
            }
            if nums[i] < 0 {
                nums[i] *= -1;
                k -= 1;
            }
        }

        // 取反偶数次还是原来的数
        if k % 2 == 1 {
            // 将最小的正数取反
            *nums.last_mut().unwrap() *= -1;
        }
        // 求和
        nums.iter().sum()
    }

    pub fn largest_sum_after_k_negationss(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        for _ in 0..k {
            nums[0] *= -1;
            // nums[0] = 0 - nums[0];

            for i in 1..nums.len() {
                if nums[i] < nums[i - 1] {
                    nums[i] ^= nums[i - 1];
                    nums[i - 1] ^= nums[i];
                    nums[i] ^= nums[i - 1];
                } else {
                    break;
                }
            }
        }

        nums.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::largest_sum_after_k_negations(vec![-3, -1, -4, 4, 2, 3], 1);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
