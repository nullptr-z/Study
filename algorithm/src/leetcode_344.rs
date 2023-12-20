impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            let temp = s[l];
            s[l] = s[r];
            s[r] = temp;

            l += 1;
            r -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let mut ret = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut ret);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
