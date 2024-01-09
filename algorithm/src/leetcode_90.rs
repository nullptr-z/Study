impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtracking(nums: &[i32], result: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
            result.push(temp.to_vec());

            let mut flags = false;
            for (i, &val) in nums.iter().enumerate() {
                if flags && nums[i] == nums[i - 1] {
                    continue;
                }
                temp.push(val);
                backtracking(&nums[i + 1..], result, temp);
                flags = true;
                temp.pop();
            }
        }

        let mut temp = vec![];
        let mut result = vec![];
        nums.sort();
        backtracking(&nums, &mut result, &mut temp);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let res = Solution::subsets_with_dup([1, 2, 2].into());
        println!("【 res 】==> {:?}", res);
    }
}

pub struct Solution;
