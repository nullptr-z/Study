impl Solution {
    pub fn monotone_increasing_digits(mut n: i32) -> i32 {
        let mut nums: Vec<i32> = n.to_string().chars().map(|m| (m as i32) - 48).collect();
        for i in (1..nums.len()).rev() {
            if nums[i] < nums[i - 1] {
                nums[i - 1] -= 1;
                nums[i..].iter_mut().for_each(|m| *m = 9);
            }
        }
        let result: i32 = nums.into_iter().fold(0, |acc, num| acc * 10 + num);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::monotone_increasing_digits(100);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
