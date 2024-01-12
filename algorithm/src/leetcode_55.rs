impl Solution {
    pub fn can_jump(mut nums: Vec<i32>) -> bool {
        nums.reverse();
        let mut count = 0;
        for i in 1..nums.len() {
            let val = nums[i];
            count += 1;
            if count - val <= 0 {
                count = 0;
            }
        }

        count <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::can_jump(vec![3, 2, 1, 0, 4]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
