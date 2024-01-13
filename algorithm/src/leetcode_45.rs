impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let end = nums.len() - 1;
        let mut jump = 0;
        let mut next_index = 0;
        let mut cur_index = 0;
        for (i, step) in nums.into_iter().enumerate().take(end) {
            next_index = next_index.max(i + step as usize);
            if i == cur_index {
                cur_index = next_index;
                jump += 1;
            }
        }

        jump
    }

    pub fn jumps(nums: Vec<i32>) -> i32 {
        let len = nums.len() - 1;
        let mut count = 0;

        let mut i = 0;
        while i < len {
            let val = nums[i];
            if (i + val as usize) >= len {
                // 当前位置可以直接跳到尾部
                return count + 1;
            }
            let mut max = 0;
            let mut next_index = 0;
            // 找到当前位置可跳跃到的，最大的跳跃数位置
            let start = i + 1;
            let end = (i + val as usize).min(len);
            // 可跳跃的范围, 闭区间。
            for j in start..=end {
                // j是成递增的，后面位置比前面位置更近一步，所以 + j
                let val = nums[j] as usize + j;
                // 找出下一个最大跳跃值位置
                if val >= max {
                    max = val;
                    next_index = j;
                }
            }
            count += 1;
            i = next_index;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::jump(vec![1, 2, 3]);
        // println!("【 ret 】==> {:?}", ret);
        let ret = Solution::jump(vec![2, 3, 0, 1, 4]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::jump(vec![2, 3, 1, 1, 4]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::jump(vec![2, 1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
