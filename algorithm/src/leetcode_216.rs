impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            k: usize,
            n: i32,
            start: usize,
            sum: i32,
            part: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if sum > n {
                return;
            }
            let len = part.len();
            if len == k {
                if sum == n {
                    result.push(part.to_owned());
                }
                return;
            }
            for i in start..=9 - (k - len) + 1 {
                part.push(i as i32);
                backtracking(k, n, i + 1, sum + i as i32, part, result);
                part.pop();
            }
        }

        let mut part = vec![];
        let mut result = vec![];
        backtracking(k as usize, n, 1, 0, &mut part, &mut result);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::combination_sum3(9, 45);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
