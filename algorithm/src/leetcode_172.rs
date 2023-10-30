impl Solution {
    // 简单地说就是n里面有多少个因子5，例如10有2个5，11也有两个5，25有6个；
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n < 5 {
            return 0;
        }

        let mut count = 0;

        let mut c = n;
        while c > 0 {
            c /= 5;
            count += c;
        }

        println!("【 count 】==> {:?}", count);
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::trailing_zeroes(25);
    }
}

pub struct Solution;
