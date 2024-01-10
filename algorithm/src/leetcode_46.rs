impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(
            nums: &[i32],
            result: &mut Vec<Vec<i32>>,
            temp: &mut Vec<i32>,
            used: &mut Vec<bool>,
        ) {
            if nums.len() == temp.len() {
                result.push(temp.to_owned());
                return;
            }

            for (i, &val) in nums.iter().enumerate() {
                if used[i] {
                    continue;
                }
                temp.push(val);
                used[i] = true;
                backtracking(nums, result, temp, used);
                used[i] = false;
                temp.pop();
            }
        }

        let mut result = vec![];
        let mut temp = vec![];
        let mut used = vec![false; nums.len()];
        backtracking(&nums, &mut result, &mut temp, &mut used);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::permute([5, 4, 6, 2].into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
