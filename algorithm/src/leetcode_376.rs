use std::cmp::Ordering;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut compile = Ordering::Equal;
        let mut count = 1;
        for i in 1..nums.len() {
            match nums[i].cmp(&nums[i - 1]) {
                Ordering::Equal => {}
                Ordering::Less => {
                    if compile != Ordering::Less {
                        count += 1;
                        compile = Ordering::Less;
                    }
                }
                Ordering::Greater => {
                    if compile != Ordering::Greater {
                        count += 1;
                        compile = Ordering::Greater;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
