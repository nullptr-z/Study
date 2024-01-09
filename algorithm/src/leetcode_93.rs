impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn backtracking(s: &str, result: &mut Vec<String>, temp: &mut Vec<String>) {
            if s.is_empty() && temp.len() == 4 {
                let ip = temp.join(".");
                result.push(ip);
                return;
            }

            let end = 3.min(s.len()); // 实际上不做这个剪枝，也会因为大于255被过滤掉
            for i in 0..end {
                let sub_str = s[0..i + 1].to_string();
                if i > 0 && &s[0..1] == "0" || sub_str.parse::<i32>().unwrap() > 255 {
                    return;
                }
                temp.push(sub_str);
                backtracking(&s[i + 1..], result, temp);
                temp.pop();
            }
        }

        let mut result = vec![];
        let mut temp = vec![];
        backtracking(&s, &mut result, &mut temp);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::restore_ip_addresses("25525511135".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
