//

pub struct Solution;

impl Solution {
    pub fn rob(mut nums: Vec<i32>) -> i32 {
        let mut len = nums.len();
        if len == 1 {
            return nums[0];
        } else if len == 2 {
            return nums[0].max(nums[1]);
        }

        let mut pre =0;
        let mut cur = 0;
        let mut i = 0;
        while i < len {
            let sum = cur.max(nums[i] + pre);
            pre = cur;
            cur = sum;

            i += 1;
        }
        let max = cur.max(pre);
        println!("【 max 】==> {:?}", max);

        println!("【 max 】==> {:?}", cur);

        cur
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::rob(vec![2,1,1,2]);
    }
}
// 2 1 1 2
// 2 7 9 3 1
