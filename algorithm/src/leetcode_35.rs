impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let i = (end + start) / 2;
            let val = nums[i];
            println!("【 start 】==> {:?} {}", start, end);
            if val == target {
                return i as i32;
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

        start as i32 // 返回插入位置
    }
}
#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::search_insert(vec![1, 3, 5, 6], 0);
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
