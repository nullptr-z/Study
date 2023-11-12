impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len() - 1;
        let (mut L, mut R, mut max) = (0, len, 0);

        while L < R {
            if height[R] >= height[L] {
                max = max.max((R - L) as i32 * height[L]);
                L += 1;
            } else {
                max = max.max((R - L) as i32 * height[R]);
                R -= 1;
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        let ret = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 25, 7]);

        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
