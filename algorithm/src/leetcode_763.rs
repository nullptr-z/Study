impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for (i, val) in s.chars().enumerate() {
            let index = &s[0..i].find(val);
            if let Some(idx) = index {
                let mut sum = 0;
                let mut find_index = 0;
                let mut c_r = 0;
                for (j, &r) in result.iter().enumerate() {
                    c_r += r;
                    if (*idx as i32) < c_r || sum > 0 {
                        if sum == 0 {
                            find_index = j;
                        }
                        sum += r;
                    }
                }
                sum += 1;
                result = result[..find_index].to_owned();
                result.push(sum);
            } else {
                result.push(1)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::partition_labels("ababcbacadefegdehijhklij".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
