use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &[i32], result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
            if temp.len() >= 2 {
                result.push(temp.to_owned());
            }
            if nums.is_empty() {
                return;
            }

            let mut flags = HashSet::new();
            for (i, &val) in nums.iter().enumerate() {
                if !flags.insert(val) || !temp.is_empty() && val < *temp.last().unwrap() {
                    continue;
                }
                temp.push(val);
                backtracking(&nums[i + 1..], result, temp);
                temp.pop();
            }
        }

        let mut result = vec![];
        let mut temp = vec![];
        backtracking(&nums, &mut result, &mut temp);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret =
            Solution::find_subsequences([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1].into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
