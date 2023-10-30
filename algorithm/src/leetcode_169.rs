use rand::Rng;

impl Solution {
    // 摩尔投票法
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        let mut cand_num = nums[0];
        let mut count = 1;
        for val in nums.iter() {
            if *val == cand_num {
                count += 1;
            } else {
                count -= 1;
                if count == 0 {
                    count = 1;
                    cand_num = *val;
                }
            }
        }

        cand_num
    }

    // 排序取中间的
    pub fn majority_element_sort(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort();
        nums[len / 2]
    }

    // 随机法，概率大于50%
    // 还可以从半数的Nums中依次取一个，至少有一个是对的
    pub fn majority_element_rand(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        loop {
            let rand = rand::thread_rng().gen_range(0..len);
            let majority = nums[rand];
            let mut count = 0;
            for val in nums.iter() {
                if *val == majority {
                    count += 1;
                }
            }
            if count > len / 2 {
                return majority;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
    }
}

pub struct Solution;
