use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut set = HashSet::new();

        for v in nums1 {
            set.insert(v);
        }

        for v in nums2 {
            if set.get(&v).is_some() {
                set.remove(&v);
                result.push(v);
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
        let ret = Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
