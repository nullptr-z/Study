impl Solution {
    // 二分法，logn时间复杂度
    // 每次将n减少一倍，对pow(n)的结果相乘
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let p = Self::my_pow_helper(x, n.abs());
        if n >= 0 {
            p
        } else {
            1.0 / p
        }
    }

    fn my_pow_helper(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let p = Self::my_pow_helper(x, n / 2);
        let pp = p * p;
        if n % 2 == 0 {
            pp
        } else {
            pp * x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::my_pow(2.0, 2);
    }
}

pub struct Solution;
