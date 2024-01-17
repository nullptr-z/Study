impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        println!("【 intervals 】==> {:?}", intervals);
        let mut result: Vec<Vec<i32>> = vec![intervals[0].to_owned()];

        for i in 1..intervals.len() {
            let last = result.last_mut().unwrap();
            if intervals[i][1] <= last[1] {
                continue;
            }
            if intervals[i][0] <= last[1] {
                last[1] = intervals[i][1]
            } else {
                result.push(intervals[i].to_owned())
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
        let ret = Solution::merge(
            [[2, 3], [4, 5], [6, 7], [8, 9], [1, 10]]
                .iter()
                .map(|m| m.to_vec())
                .collect::<Vec<Vec<i32>>>(),
        );
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
