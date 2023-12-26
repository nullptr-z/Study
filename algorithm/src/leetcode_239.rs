use crate::queues::Queue;
use crate::single_queue::SingleQueue;

impl Solution {
    // 单调队列
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![];
        let mut queue = SingleQueue::new();

        for v in &nums[0..k] {
            queue.push(*v);
        }
        result.push(queue.get_max());

        for i in k..nums.len() {
            queue.pop(nums[i - k]);
            queue.push(nums[i]);
            result.push(queue.get_max())
        }

        result
    }
    // 超时
    pub fn max_sliding_windows(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue = Queue::default();
        let mut result = vec![];
        let mut max = i32::MIN;

        for v in 0..k {
            max = max.max(nums[v]);
            queue.push(nums[v]);
        }
        result.push(max);

        for v in &nums[k..] {
            let v = *v;
            let top_val = queue.pop();
            queue.push(v);
            let pre_max = *result.last().unwrap();
            let max = match top_val.cmp(&pre_max) {
                std::cmp::Ordering::Equal => {
                    if pre_max <= v {
                        v
                    } else {
                        let mut max = *queue.0.front().unwrap();
                        for v in &queue.0 {
                            max = max.max(*v);
                        }
                        max
                    }
                }
                _ => pre_max.max(v),
            };
            result.push(max);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
