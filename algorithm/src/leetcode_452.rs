//

impl Solution {
    pub fn find_min_arrow_shots(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_by_key(|interval| interval[0]);

        let mut right_note = intervals[0][1];
        let mut count_arrow = 1;
        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= right_note {
                right_note = right_note.min(interval[1]);
            } else {
                count_arrow += 1;
                right_note = interval[1];
            }
        }

        count_arrow
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let result =
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]);
        let result = Solution::find_min_arrow_shots(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10],
        ]);

        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
