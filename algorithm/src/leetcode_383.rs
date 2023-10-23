//

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut resource = HashMap::with_capacity(magazine.len());
        for value in magazine.as_bytes().iter() {
            let that = resource.entry(value);
            that.and_modify(|v| *v += 1).or_insert(1);
        }

        for value in ransom_note.as_bytes().iter() {
            if resource.get(value).is_none() {
                return false;
            }
            let that = resource.get_mut(value);
            if let Some(v) = that {
                *v -= 1;
                if *v == 0 {
                    resource.remove(value);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let result = Solution::can_construct("aa".to_string(), "aab".to_string());
        println!("【 result 】==> {:?}", result);
    }
}

pub struct Solution;
