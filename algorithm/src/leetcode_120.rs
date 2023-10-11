//

pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut len = triangle.len();
        let mut dp = triangle.last().unwrap().to_owned();
        len -= 1;

        while len > 0 {
            len -= 1;
            for (i, val) in triangle[len].iter().enumerate() {
                dp[i] = (dp[i] + val).min(dp[i + 1] + val);
            }
        }

        // println!("【 dp 】==> {:?}", dp);
        dp[0]
    }
}

fn pack(arr: Vec<i32>) -> Vec<i32> {
    let mut pack_arr = vec![0; arr.len() - 1];
    for i in 0..arr.len() - 1 {
        pack_arr[i] = arr[i] + arr[i + 1];
    }

    pack_arr
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]);
    }
}
