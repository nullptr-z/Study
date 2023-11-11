use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        println!("【 nums 】==> {:?}", nums);

        for i in 0..nums.len() {
            if i != 0 && nums[i - 1] == nums[i] {
                // 对于不同下标相同的值跳过
                continue;
            }
            if nums[i] > 0 {
                // 左指针大于0那就说明不可能存合计0的选项了
                return result;
            }
            let mut M = i + 1;
            let mut E = nums.len() - 1;
            let sum = 0 - nums[i];
            while M < E {
                match (nums[M] + nums[E]).cmp(&sum) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[M], nums[E]]);
                        while M < E && nums[M] == nums[M + 1] {
                            // 对于不同下标相同的值跳过
                            M += 1;
                        }
                        while M < E && nums[E] == nums[E - 1] {
                            // 对于不同下标相同的值跳过
                            E -= 1;
                        }
                        M += 1;
                        E -= 1;
                    }
                    Ordering::Greater => {
                        E -= 1;
                    }
                    Ordering::Less => {
                        M += 1;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
