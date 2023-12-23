impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for i in s.chars() {
            match i {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ' ' => continue,
                _ => {
                    if Some(i) != stack.pop() {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::is_valid("()[] {}".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
