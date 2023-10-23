//

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // 长度相同
        // 使用所有字母，字符顺序不同
        // 非字母异位词Ascii之和也可能相同
        let mut map = HashMap::new();

        for value in strs.iter() {
            let mut key = value.clone().into_bytes();
            key.sort();
            let that = map.entry(key);
            that.and_modify(|m: &mut Vec<String>| m.push(value.to_owned()))
                .or_insert(vec![value.to_owned()]);
        }

        let values: Vec<Vec<String>> = map.values().cloned().collect();

        values
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .iter()
                .map(|m| m.to_string())
                .collect(),
        );
    }
}

pub struct Solution;
