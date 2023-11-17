impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        for val in nums {
            ones = (ones ^ val) & !twos;
            twos = (twos ^ val) & !ones;
        }
        return ones;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::single_number(vec![2, 2, 3, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
