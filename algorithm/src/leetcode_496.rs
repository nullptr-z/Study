use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // 实际作用，用来找存入 result 的下标位置，也是这道题不容易想到的
        // 除此之外就是对 nums2，纯单调栈逻辑
        let kv: HashMap<&i32, usize> = nums1
            .iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<HashMap<_, _>>();

        let mut result = vec![-1; nums1.len()];
        let mut stack = vec![];
        for (i, &val) in nums2.iter().enumerate() {
            while let Some(&last_idx) = stack.last() {
                if nums2[last_idx] > val {
                    break;
                }
                stack.pop().unwrap();

                // 找到一个更小的值，map 中找到存放他的位置
                if let Some(&map_idx) = kv.get(&nums2[last_idx]) {
                    result[map_idx] = val;
                }
            }
            stack.push(i)
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
