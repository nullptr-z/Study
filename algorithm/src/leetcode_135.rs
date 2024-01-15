impl Solution {
    pub fn candy(mut ratings: Vec<i32>) -> i32 {
        let mut result = vec![1; ratings.len()];
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                result[i] = result[i - 1] + 1;
            }
        }

        ratings.reverse();
        result.reverse();
        for i in 1..ratings.len() {
            if ratings[i] > ratings[i - 1] {
                result[i] = result[i].max(result[i - 1] + 1);
            }
        }

        result.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::candy(vec![1, 2, 2]);
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::candy(vec![1, 0, 2]);
        // println!("【 ret 】==> {:?}", ret);
        let ret = Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]);
        println!("【 ret 】==> {:?}", ret);
        let ret = Solution::candy(vec![1, 3, 4, 5, 2]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
