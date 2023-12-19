impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if nums.len() < 4 {
            return result;
        }

        nums.sort();

        for k in 0..nums.len() - 3 {
            // 剪枝：排序后第一个元素大0，就不可能得到解
            if nums[k] >= 0 && nums[k] > target {
                break;
            }

            // 去重
            if k > 0 && nums[k] == nums[k - 1] {
                continue;
            }

            for i in (k + 1)..nums.len() - 2 {
                // 去重
                if i > (k + 1) && nums[i] == nums[i - 1] {
                    continue;
                }

                let pre_two_sum: i32 = nums[i] + nums[k];
                // 剪枝：排序后第一个元素大0，就不可能得到解
                if pre_two_sum >= 0 && pre_two_sum > target {
                    break;
                }

                let (mut l, mut r) = (i + 1, nums.len() - 1);
                while l < r {
                    let sum: i64 = (nums[l] as i64) + (pre_two_sum as i64) + (nums[r] as i64);
                    match sum.cmp(&(target as i64)) {
                        std::cmp::Ordering::Less => l += 1,
                        std::cmp::Ordering::Greater => r -= 1,
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[i], nums[k], nums[l], nums[r]]);

                            while l < r && nums[l + 1] == nums[l] {
                                l += 1;
                            }
                            while r > l && nums[r - 1] == nums[r] {
                                r -= 1;
                            }

                            l += 1;
                            r -= 1;
                        }
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
        let ret = Solution::four_sum(
            vec![1000000000, 1000000000, 1000000000, 1000000000],
            -294967296,
        );
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
