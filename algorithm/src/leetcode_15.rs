use std::cmp::Ordering;
use std::collections::HashMap;

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

    /// 超时了，需要想办法解决重复问题
    pub fn three_sums(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut result: Vec<Vec<i32>> = vec![];
        let len = nums.len() - 1;

        let mut one = 0;
        let mut two = 1;
        let mut three = 2;
        loop {
            let sum = nums[one] + nums[two] + nums[three];
            if sum == 0 {
                result.push(vec![nums[one], nums[two], nums[three]]);
            }

            if one == len - 2 {
                let mut map = HashMap::new();
                for mut item in result {
                    item.sort();
                    let s = format!("{:?}", item);
                    map.entry(s).or_insert(item);
                }
                let result = map.into_iter().map(|v| v.1).collect();
                return result;
            } else if two == len - 1 {
                one += 1;
                two = one + 1;
                three = two + 1;
            } else if three == len {
                two = two + 1;
                three = two + 1;
            } else {
                three += 1;
            }
        }
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
