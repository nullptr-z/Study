impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for v in s.chars() {
            if stack.is_empty() || Some(&v) != stack.last() {
                stack.push(v);
            } else {
                stack.pop();
            }
        }

        stack.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::remove_duplicates("abbaca".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
