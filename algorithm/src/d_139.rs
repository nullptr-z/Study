use std::collections::HashSet;

impl Solution {
    // 回溯+贪心，没有贪过
    pub fn word_breaks(s: String, word_dict: Vec<String>) -> bool {
        let dict = word_dict.into_iter().collect::<HashSet<String>>();
        let len = s.len();
        let mut left = 0;
        let mut right = 1;
        let mut dp = vec![];
        while left < right {
            while right <= len {
                if let Some(_v) = dict.get(&s[left..right]) {
                    dp.push((left, right));
                    left = right;
                    if left == len {
                        return true;
                    }
                }
                if left == 0 && right == len {
                    return false;
                }
                if right == len {
                    if let Some(d) = dp.pop() {
                        (left, right) = d;
                    }
                }
                right += 1
            }
        }

        false
    }

    // 计算连续区间，好像也不行
    pub fn word_break_c(s: String, word_dict: Vec<String>) -> bool {
        let mut s_range: Vec<(usize, usize)> = vec![];
        s_range.sort();
        for val in word_dict {
            if let Some(start) = s.find(&val) {
                s_range.push((start, start + val.len()))
            };
        }
        println!("【 s_range 】==> {:?}", s_range);
        false
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for cap in 1..dp.len() {
            for word in word_dict.iter() {
                if cap >= word.len() {
                    let start = cap - word.len();
                    if s[start..cap].eq(word) && dp[start] {
                        dp[cap] = true;
                        break;
                    }
                }
            }
        }
        dp[s.len()]
    }

    pub fn word_break_set(s: String, word_dict: Vec<String>) -> bool {
        let mut dp = vec![false; s.len() + 1];
        let dicts = word_dict.into_iter().collect::<HashSet<String>>();
        dp[0] = true;
        for cap in 1..dp.len() {
            for j in 0..cap {
                if let Some(_word) = dicts.get(&s[j..cap]) {
                    if dp[j] {
                        dp[cap] = true;
                        break;
                    }
                }
            }
        }
        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::word_break(
            "aaaaaaa".to_string(),
            vec!["aaaa", "aaa"].iter().map(|m| m.to_string()).collect(),
        );
        println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::word_break(
        //     "catsandog".to_string(),
        //     vec!["cats", "dog", "sand", "and", "cat"]
        //         .iter()
        //         .map(|m| m.to_string())
        //         .collect(),
        // );
        // println!("【 ret 】==> {:?}", ret);
        // let ret = Solution::word_break(
        //     "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
        //     vec!["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]
        //         .iter()
        //         .map(|m| m.to_string())
        //         .collect(),
        // );
        // println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;

// for right in 0..=len {
//     println!("【 left 】==> {:?} {}", left, right);
//     if let Some(_v) = dict.get(&s[left..right]) {
//         println!("【 _v 】==> {:?}", _v);
//         left = right;
//         if left == len {
//             return true;
//         }
//     }
// }
