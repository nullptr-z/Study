use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let (first, last) = digits.split_at(1);
        let result = Self::letter_combinations(last.to_string());
        let mut ret = vec![];
        for key in get_letters(first) {
            if result.is_empty() {
                ret.push((*key).to_string());
            }
            for val in &result {
                ret.push(format!("{}{}", key, val));
            }
        }
        ret
    }
}

fn get_letters(c: &str) -> Vec<String> {
    let mut map = HashMap::new();
    map.insert("2", vec!["a", "b", "c"]);
    map.insert("3", vec!["d", "e", "f"]);
    map.insert("4", vec!["g", "h", "i"]);
    map.insert("5", vec!["j", "k", "l"]);
    map.insert("6", vec!["m", "n", "o"]);
    map.insert("7", vec!["p", "q", "r", "s"]);
    map.insert("8", vec!["t", "u", "v"]);
    map.insert("9", vec!["w", "x", "y", "z"]);

    let ret = map.get(&c).unwrap().iter().map(|v| v.to_string()).collect();

    ret
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::letter_combinations("23".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
