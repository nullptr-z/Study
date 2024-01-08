impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtracking(
            candidates: &[i32],
            subset: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            target: i32,
            sum: i32,
        ) {
            if sum == target {
                result.push(subset.to_vec());
                return;
            }

            let mut flags = false;
            for (i, &val) in candidates.iter().enumerate() {
                if (sum + val) > target {
                    return;
                }
                if flags && candidates[i] == candidates[i - 1] {
                    continue;
                }
                subset.push(val);
                backtracking(&candidates[(i + 1)..], subset, result, target, sum + val);
                subset.pop();
                flags = true;
            }
        }

        let mut candidates = candidates;
        candidates.sort();

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
        let ret = Solution::combination_sum2([10, 1, 2, 7, 6, 1, 5].into(), 8);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
