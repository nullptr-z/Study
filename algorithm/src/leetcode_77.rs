impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut temp = vec![];

        fn backtrack(start: i32, k: i32, n: i32, temp: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if k == 0 {
                ret.push(temp.clone());
                return;
            }

            for i in start..=n {
                temp.push(i);
                println!("【 temp 】==> {:?}", temp);
                backtrack(i + 1, k - 1, n, temp, ret);
                temp.pop();
            }
        }

        backtrack(1, k, n, &mut temp, &mut ret);

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::combine(4, 3);
    }
}

pub struct Solution;
