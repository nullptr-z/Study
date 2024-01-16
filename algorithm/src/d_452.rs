impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort();
        let mut count = 1;
        let mut start = points[0][1];
        for i in 1..points.len() {
            if points[i][0] > start {
                count += 1;
                start = points[i][1];
                continue;
            }
            start = start.min(points[i][1])
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::find_min_arrow_shots(
            vec![
                [9, 12],
                [1, 9],
                [1, 10],
                [4, 11],
                [8, 12],
                [3, 9],
                [6, 9],
                [6, 7],
            ]
            .iter()
            .map(|m| m.to_vec())
            .collect(),
        );
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
