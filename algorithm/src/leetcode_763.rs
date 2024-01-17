impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut left = 0;
        let mut right = 0;
        for (i, val) in s.chars().enumerate() {
            right = right.max(1 + s.rfind(val).unwrap() as i32);
            if i == (right - 1) as usize {
                result.push(right - left);
                left += right - left;
                right = i as i32;
            }
        }

        result
    }

    pub fn partition_labels_has(s: String) -> Vec<i32> {
        let mut has = vec![0; 26];
        for (i, &val) in s.as_bytes().iter().enumerate() {
            has[(val - b'a') as usize] = i;
        }

        let mut result: Vec<i32> = vec![];
        let mut left = 0;
        let mut right = 0;
        for (i, &val) in s.as_bytes().iter().enumerate() {
            right = right.max(1 + has[(val - b'a') as usize]);
            if i == (right - 1) as usize {
                result.push((right - left) as i32);
                left += right - left;
                right = i;
            }
        }

        result
    }

    pub fn partition_labelsy(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0];

        for (i, val) in s.chars().enumerate() {
            // 找到当前字符的最远距离的相同字符，假设这就是一个完整的区间
            let index = 1 + s.rfind(val).unwrap() as i32;
            let last = *result.last().unwrap();
            // 如果区间内的字符最远距离大于当前区间最远距离，刷新最远距离
            *result.last_mut().unwrap() = last.max(index);

            let last = *result.last().unwrap();
            if i != s.len() - 1 && i == (last - 1) as usize {
                // 这是一个完整的区间了，之后不会再与后面的区间重叠
                // 开启一个新的区间
                result.push(0);
            }
        }
        // 存的下标位置，减去上位置就是数量
        // 把逻辑这段逻辑分离出来，更易于阅读代码
        for i in (1..result.len()).rev() {
            result[i] -= result[i - 1];
        }

        result
    }

    pub fn partition_labelsz(s: String) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut interval: Vec<&str> = vec![];

        for (j, val) in s.chars().enumerate() {
            let mut len = 0;
            let mut is_find = false;
            for i in 0..interval.len() {
                let it = interval[i];
                if let Some(_idx) = it.find(val) {
                    interval.truncate(i);
                    interval.push(&s[len..=j]);
                    is_find = true;
                    break;
                }
                len += it.len();
            }
            if !is_find {
                interval.push(&s[j..j + 1])
            }
        }

        for it in interval {
            result.push(it.len() as i32)
        }

        result
    }

    pub fn partition_labelss(s: String) -> Vec<i32> {
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
        let ret = Solution::partition_labels_has("ababcbacadefegdehijhklij".into());
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
