//

pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut f = vec![0; (n + 1) as usize];
        for i in 1..=n {
            let mut minn = i32::MAX;
            let mut j = 1;
            while j * j <= i {
                minn = minn.min(f[(i - j * j) as usize]);
                j += 1;
            }
            f[i as usize] = minn + 1;
        }

        f[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::num_squares(67);
        assert_eq!(result, 3);

        // let result = Solution::num_squares(4);
        // assert_eq!(result, 1);

        // let result = Solution::num_squares(2);
        // assert_eq!(result, 2);

        // let result = Solution::num_squares(17);
        // assert_eq!(result, 2);

        // let result = Solution::num_squares(16);
        // assert_eq!(result, 1);

        // let result = Solution::num_squares(12);
        // assert_eq!(result, 3);

        // let result = Solution::num_squares(13);
        // assert_eq!(result, 2);
    }
}
