impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut LH = vec![0; len];
        let mut RH = vec![0; len];
        // 柱子左右两侧
        LH[0] = height[0];
        RH[len - 1] = height[len - 1];

        // 柱子左边最高柱子
        for i in 1..len {
            LH[i] = LH[i - 1].max(height[i]);
        }
        // 柱子右边最高柱子
        for i in (0..len - 1).rev() {
            RH[i] = RH[i + 1].max(height[i]);
        }

        let mut sum = 0;
        for i in 0..len {
            sum += LH[i].min(RH[i]) - height[i];
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
