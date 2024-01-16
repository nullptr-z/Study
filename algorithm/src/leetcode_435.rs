impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        intervals.sort_by_key(|m| m[1]);
        let mut end = intervals[0][1];
        for i in 1..intervals.len() {
            if intervals[i][0] < end {
                result += 1;
                continue;
            }
            end = intervals[i][1];
        }
        result as i32
    }

    pub fn erase_overlap_intervalss(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        intervals.sort();
        let mut end = intervals[0][1];
        for i in 1..intervals.len() {
            if intervals[i][0] < end {
                result += 1;
                end = end.min(intervals[i][1]);
            } else {
                end = intervals[i][1];
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::erase_overlap_intervals(
        //     vec![[1, 2], [2, 3], [3, 4], [1, 3]]
        //         .iter()
        //         .map(|m| m.to_vec())
        //         .collect(),
        // );
        let ret = Solution::erase_overlap_intervals(
            vec![
                [-52, 31],
                [-73, -26],
                [82, 97],
                [-65, -11],
                [-62, -49],
                [95, 99],
                [58, 95],
                [-31, 49],
                [66, 98],
                [-63, 2],
                [30, 47],
                [-40, -26],
            ]
            .iter()
            .map(|m| m.to_vec())
            .collect(),
        );
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
