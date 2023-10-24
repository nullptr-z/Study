//

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        let mut min_distance: usize = usize::MAX;
        for (i, val) in nums.iter().enumerate() {
            map.entry(val)
                .and_modify(|m: &mut (usize, usize)| {
                    let distance = i - m.0;
                    m.0 = i;
                    m.1 = distance.min(m.1);
                    min_distance = min_distance.min(m.1);
                })
                .or_insert((i, usize::MAX));
        }

        min_distance <= k as usize
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2);
    }
}

pub struct Solution;
