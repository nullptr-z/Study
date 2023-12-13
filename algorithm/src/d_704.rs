impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut start, mut end) = (0, nums.len());

        while start < end {
            let mid = (start >> 1) + (end >> 1);
            match target.cmp(&nums[mid]) {
                std::cmp::Ordering::Greater => start = mid + 1,
                std::cmp::Ordering::Less => end = mid,
                std::cmp::Ordering::Equal => return mid as i32,
            };
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let r = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        println!("【 r 】==> {:?}", r);
    }
}

pub struct Solution;
