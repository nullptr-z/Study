//

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chart: &[u8] = s.as_bytes();
        let len = s.len();

        let mut range = 0..0;

        let mut index: usize = 0;
        while index < len {
            let mut end_ptr = len;
            // let mut long = 0;
            while end_ptr > index {
                let range_long = end_ptr - index;
                let result = palindrome(&chart[index..end_ptr]);
                if result && range_long > range.len() {
                    range = index..end_ptr;
                    // index += range_long - 1;
                }
                end_ptr -= 1;
            }
            index += 1;
            if len - index < range.len() {
                break;
            }
        }

        let result = String::from_utf8(chart[range].to_vec()).unwrap();
        // println!("【 result 】==> {:?}", result);

        result
    }
}

pub fn palindrome(chart: &[u8]) -> bool {
    let end = chart.len() - 1;
    let mut flag = true;
    for i in 0..=end {
        if chart[i] != chart[end - i] {
            flag = false;
            break;
        }
    }

    flag
}

#[cfg(test)]
mod tests {
    use super::{palindrome, Solution};

    #[test]
    fn should_work() {
        Solution::longest_palindrome("abacab".to_string());
    }

    #[test]
    fn palindrome_should_work() {
        palindrome("babab".as_bytes());
    }
}
