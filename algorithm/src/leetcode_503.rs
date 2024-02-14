impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums.len() * 2];
        let double = nums.repeat(2);
        let mut stack = vec![];
        for (i, &val) in double.iter().enumerate() {
            while let Some(idx) = stack.last() {
                if double[*idx] < val {
                    result[*idx] = val;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i)
        }

        result[0..nums.len()].into()
        // result.into_iter().take(nums.len()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::next_greater_elements(vec![1, 2, 3, 4, 3]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
