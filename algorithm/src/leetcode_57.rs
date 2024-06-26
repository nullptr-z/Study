//

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut max = 0;
        let mut arr = vec![-1; 100001];
        intervals.push(new_interval);
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
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
    }
}

pub struct Solution;
