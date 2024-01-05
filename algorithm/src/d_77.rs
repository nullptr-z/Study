impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut subset = vec![];

        Self::backtracking(n as usize, k as usize, &mut result, &mut subset, 1);
        result
    }

    pub fn backtracking(
        n: usize,
        k: usize,
        result: &mut Vec<Vec<i32>>,
        subset: &mut Vec<i32>,
        start: usize,
    ) {
        let len = subset.len();
        if len == k {
            result.push(subset.to_owned());
            return;
        }
        for i in start..=n - (k - len - 1) {
            subset.push(i as i32);
            Self::backtracking(n, k, result, subset, i + 1);
            subset.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::combine(4, 2);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
