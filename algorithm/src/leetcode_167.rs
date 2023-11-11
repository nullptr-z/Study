impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pre = 0;
        let mut end = numbers.len() - 1;

        loop {
            if numbers[pre] + numbers[end] > target {
                end -= 1;
            } else if numbers[pre] + numbers[end] < target {
                pre += 1;
            } else {
                return vec![pre as i32 + 1, end as i32 + 1];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::two_sum(vec![2, 7, 11, 15], 9);
    }
}

pub struct Solution;
