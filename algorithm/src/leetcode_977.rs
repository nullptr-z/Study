impl Solution {
    /// 非递减序列：有序列，递增
    /// 平方：负数变成正数
    /// 由此最大的数一定在左右两端之一
    /// 坑：不要进行原数组的两数交换，一个小的值可能被换到平方最大值的前面
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut result = vec![0; len];

        let (mut l, mut r, mut cur) = (0, len - 1, len);

        while cur != 0 {
            cur -= 1;
            if nums[l].abs() > nums[r].abs() {
                result[cur] = nums[l] * nums[l];
                l += 1;
            } else {
                result[cur] = nums[r] * nums[r];
                r -= 1;
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
        let res = Solution::sorted_squares(vec![-4, -1, 0, 3, 10]);
        // let res = Solution::sorted_squares(vec![-5, -3, -2, -1]);
        println!("【 res 】==> {:?}", res);
    }
}

pub struct Solution;
