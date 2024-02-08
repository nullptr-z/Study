impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = vec![];
        for (i, &temper) in temperatures.iter().enumerate() {
            while let Some(&last_idx) = stack.last() {
                if temper > temperatures[last_idx] {
                    result[last_idx] = (i - last_idx) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i)
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
