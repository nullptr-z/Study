impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::with_capacity(tokens.len());

        for s in tokens {
            match s.as_str() {
                "+" => {
                    let r = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += r;
                }
                "-" => {
                    let r = stack.pop().unwrap();
                    *stack.last_mut().unwrap() -= r;
                }
                "*" => {
                    let r = stack.pop().unwrap();
                    *stack.last_mut().unwrap() *= r;
                }
                "/" => {
                    let r = stack.pop().unwrap();
                    *stack.last_mut().unwrap() /= r;
                }
                _ => stack.push(s.parse().unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::eval_rpn(vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ]);
        println!("【 ret 】==> {:?}", ret);
        let values = vec!["4", "13", "5", "/", "+"]
            .iter()
            .map(|m| m.to_string())
            .collect();
        let ret = Solution::eval_rpn(values);
        println!("【 ret 】==> {:?}", ret);
        let values = vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]
        .iter()
        .map(|m| m.to_string())
        .collect();
        let ret = Solution::eval_rpn(values);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
