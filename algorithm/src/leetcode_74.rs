impl Solution {
    // 还可以把二维当成一维来二分
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut start = 0;
        let mut end = matrix.len();

        while start < end {
            let i = (start + end) / 2;

            if matrix[i][0] == target {
                return true;
            } else if matrix[i][0] < target {
                start = i + 1;
            } else {
                end = i;
            }
        }

        if start == 0 {
            return false;
        }

        Self::search_insert(&matrix[start - 1], target)
    }

    pub fn search_insert(nums: &Vec<i32>, target: i32) -> bool {
        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let i = (end + start) / 2;
            let val = nums[i];
            if val == target {
                return true;
            } else if val > target {
                if i > 0 {
                    end = i;
                } else {
                    end = 0;
                }
            } else {
                start = i + 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn should_work() {
        // let result = Solution::search_matrix(vec![vec![1]], 1);
        // println!("【 result 】==> {:?}", result);
        let result = Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
        );
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
