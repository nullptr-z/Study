//

pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut max = 0;
        let mut arr = vec![-1; 10001];
        for item in intervals.iter() {
            max = max.max(item[1]);
            let idx = item[0] as usize;
            arr[idx] = arr[idx].max(item[1]);
        }

        let mut start_ptr = -1;
        let mut end_ptr = -1;
        let mut i = 0;
        while i <= max {
            let ii = i as usize;
            if arr[ii] != -1 {
                if start_ptr == -1 && end_ptr == -1 {
                    start_ptr = i;
                    end_ptr = arr[ii];
                }
                if i <= end_ptr && arr[ii] > end_ptr {
                    end_ptr = arr[ii];
                }
            }
            if i == end_ptr {
                result.push(vec![start_ptr, end_ptr]);
                start_ptr = -1;
                end_ptr = -1;
            }
            i += 1;
        }
        // println!("【 arr 】==> {:?}", arr);

        result
    }

    // 需要排序
    pub fn mergesc(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Default::default();

        intervals.sort_by(|f, b| b[1].cmp(&f[1]));

        for item in intervals.iter() {
            let range = result.iter_mut().find(|m| {
                return item[1] >= m[0] && item[0] <= m[1];
            });
            match range {
                Some(m) => {
                    m[1] = m[1].max(item[1]);
                    m[0] = m[0].min(item[0]);
                }
                None => result.push(item.clone()),
            }
        }

        result
    }

    // 排序优化版
    pub fn merges(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }

        let mut intervals = intervals;
        intervals.sort_by_key(|interval| interval[0]);

        let mut result = vec![intervals[0].clone()];

        for interval in intervals.into_iter().skip(1) {
            let last = result.last_mut().unwrap();
            if interval[0] <= last[1] {
                last[1] = last[1].max(interval[1]);
            } else {
                result.push(interval);
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
        // let result = Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]);
        // let result = Solution::merge(vec![vec![1, 4], vec![4, 5]]);
        // let result = Solution::merge(vec![vec![1, 4], vec![0, 1]]);
        // let result = Solution::merge(vec![
        //     vec![2, 3],
        //     vec![4, 5],
        //     vec![6, 7],
        //     vec![8, 9],
        //     vec![1, 10],
        // ]);
        let result = Solution::merge(vec![
            vec![2, 3],
            vec![5, 5],
            vec![2, 2],
            vec![3, 4],
            vec![3, 4],
        ]);
        // let result = Solution::merge(vec![vec![1, 4], vec![0, 0]]);

        println!("【 result 】==> {:?}", result);
    }
}
