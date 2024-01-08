impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            candidates: &[i32],
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            target: i32,
            sum: i32,
        ) {
            if sum > target {
                return;
            }
            if sum == target {
                result.push(subset.to_vec())
            }

            for (i, &val) in candidates.iter().enumerate() {
                subset.push(val);
                backtracking(&candidates[i..], subset, result, target, sum + val);
                subset.pop();
            }
        }

        let mut subset = vec![];
        let mut result = vec![];
        backtracking(&candidates, &mut subset, &mut result, target, 0);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::combination_sum([2, 3, 6, 7].into(), 7);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
