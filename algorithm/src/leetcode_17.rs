use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        Self::iter(digits.as_bytes())
    }

    fn iter(byt: &[u8]) -> Vec<String> {
        let mut ret = vec![];

        if byt.len() > 0 {
            let rets = Self::iter(&byt[1..]);
            for cur in get_letters(byt[0]) {
                if rets.len() == 0 {
                    ret.push(cur.to_string())
                }
                for next in &rets {
                    ret.push(format!("{}{}", cur, next))
                }
            }
        }

        ret
    }
}

fn get_letters(c: u8) -> Vec<String> {
    let mut map = HashMap::new();
    map.insert('2', vec!["a", "b", "c"]);
    map.insert('3', vec!["d", "e", "f"]);
    map.insert('4', vec!["g", "h", "i"]);
    map.insert('5', vec!["j", "k", "l"]);
    map.insert('6', vec!["m", "n", "o"]);
    map.insert('7', vec!["p", "q", "r", "s"]);
    map.insert('8', vec!["t", "u", "v"]);
    map.insert('9', vec!["w", "x", "y", "z"]);

    let ret = map
        .get(&(c as char))
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect();

    ret
}

fn get_letter(c: u8, s: u8) -> u8 {
    let i = c - 50;
    let ret = (c + 47) + ((i * 2) as u8) + s;

    ret
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::letter_combinations("23".to_string());
    }
}

pub struct Solution;
