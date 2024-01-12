impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        for val in nums {
            sum += val;
            max_sum = max_sum.max(sum);
            sum = sum.max(0);
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::max_sub_array(vec![-2, -1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
