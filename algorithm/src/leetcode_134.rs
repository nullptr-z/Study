impl Solution {
    // 超时
    pub fn can_complete_circuits(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut gas_count = 0;
        let len = cost.len();

        let mut i = 0;
        let mut flag = false;
        loop {
            if start == len {
                return -1;
            }
            let idx = i % len;
            gas_count += gas[idx] - cost[idx];

            if gas_count < 0 {
                start += 1;
                i = start;
                gas_count = 0;
                flag = false;
                continue;
            }

            if flag && idx == start {
                break;
            }
            flag = true;

            i += 1;
        }

        println!("【 start 】==> {:?} {}", start, i);
        start as i32
    }

    // 效率最高，最小和的下一个位置就是解
    pub fn can_complete_circuit_optimization(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_sum_index = 0;
        let mut min_sum = i32::MAX;
        let len = gas.len();
        for i in 0..len {
            sum += gas[i] - cost[i];

            if sum < min_sum {
                min_sum = sum;
                min_sum_index = i;
            }
        }

        if sum < 0 {
            -1
        } else if min_sum >= 0 {
            0
        } else {
            ((min_sum_index + 1) % len) as i32 // 获取下一个位置
        }
    }

    // 拆分写法
    pub fn can_complete_circuit_complex(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut diff = 0;
        let mut gc = vec![0; len];
        for i in 0..len {
            gc[i] = gas[i] - cost[i];
            diff += gc[i];
        }

        if diff < 0 {
            return -1;
        }

        let mut start = 0;
        let mut diff = 0;
        let mut flag = false;
        let mut count = 0;
        let mut i = 0;
        loop {
            let idx = i % len;
            diff += gc[idx];

            if diff < 0 {
                count += 1;
                if count == len {
                    return -1;
                }
                diff = 0;
                i = idx + 1;
                start = i;
                flag = false;
                continue;
            }

            if flag && idx == start {
                break;
            }

            flag = true;
            i += 1;
        }

        start as i32
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len = gas.len();
        let mut diff = 0;
        let mut total_diff = 0;
        let mut start = 0;

        for i in 0..len {
            let current_diff = gas[i] - cost[i];
            diff += current_diff;
            total_diff += current_diff;

            if diff < 0 {
                diff = 0;
                start = i + 1;
            }
        }

        if total_diff >= 0 {
            start as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
        Solution::can_complete_circuit(vec![3, 1, 1], vec![1, 2, 2]);
        Solution::can_complete_circuit(vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]);
        Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]);
        Solution::can_complete_circuit(vec![1, 2, 3, 4, 5, 5, 70], vec![2, 3, 4, 3, 9, 6, 2]);
    }
}

pub struct Solution;
