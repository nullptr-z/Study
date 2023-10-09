//

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, nn: i32) -> i32 {
        let n = (m + nn - 2) as u64;
        let mut k = (m - 1) as u64;

        if n == k {
            return 1;
        }

        if m - nn > 50 {
            k = n - k;
        }

        let result = combination(n, k);
        println!("C({},{}) = {}", n, k, result);

        result
    }
}

fn combination(n: u64, k: u64) -> i32 {
    let mut result = 1;
    for i in 1..=k {
        result = result * (n - i + 1) / i;
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::unique_paths(100, 3);
    }
}
