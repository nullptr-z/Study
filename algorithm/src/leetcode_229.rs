impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }

        let majority_size = nums.len() / 3;
        let mut majority1 = (nums[0], 1);
        let mut majority2 = (0, 0);

        for val in nums.iter().skip(1) {
            if majority1.1 == 0 && *val != majority2.0 {
                majority1.1 = 1;
                majority1.0 = *val;
                continue;
            } else if majority2.1 == 0 && *val != majority1.0 {
                majority2.1 = 1;
                majority2.0 = *val;
                continue;
            }

            if *val == majority1.0 {
                majority1.1 += 1;
            } else if *val == majority2.0 {
                majority2.1 += 1;
            } else {
                majority2.1 -= 1;
                majority1.1 -= 1;
            }
        }

        majority1.1 = 0;
        majority2.1 = 0;
        for val in nums.into_iter() {
            if val == majority1.0 {
                majority1.1 += 1;
            } else if val == majority2.0 {
                majority2.1 += 1;
            }
        }

        let mut result = vec![];
        if majority1.1 > majority_size {
            result.push(majority1.0);
        }
        if majority2.1 > majority_size {
            result.push(majority2.0);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::majority_element(vec![2, 1, 1, 3, 1, 4, 5, 6]);
        println!("【 result 】==> {:?}", result);
    }
}

struct Solution;
