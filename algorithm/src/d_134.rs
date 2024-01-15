impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut result = -1;
        let (mut sum, mut sum2) = (0, 0);

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            sum += diff;
            sum2 += diff;
            if sum2 < 0 {
                // 如果累积出现负数说明此起点无法走到最后，重置，寻找新的起点
                result = -1;
                sum2 = 0;
            }
            if diff >= 0 && result == -1 {
                // 找到一个新的起点，加油数-消耗油数大于0
                result = i as i32;
            }
        }

        // 总的加油数小于消耗油数没有可行方案
        if sum < 0 {
            return -1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::can_complete_circuit(vec![6, 1, 4, 3, 5], vec![3, 8, 2, 4, 2]);
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]);
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::can_complete_circuit(vec![2], vec![2]);
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
