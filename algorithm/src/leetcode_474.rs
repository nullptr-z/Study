//

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // 好像行不通咯
    pub fn find_max_forms(mut strs: Vec<String>, m: i32, n: i32) -> i32 {
        strs.sort_by(|a, b| a.len().cmp(&b.len()));
        println!("【 strs 】==> {:?}", strs);
        let mut len = strs.len();
        // let dp = vec![0, len];
        let mut dp = HashMap::new();
        let mut count = 0;
        let mut temp = (0, 0);

        let diff = (m.max(n) - m.min(n)) as usize;
        let min = m.min(n) as usize;
        let mut flag = false;
        let mut i = 0;
        while i < len {
            println!("【 i 】==> {:?} {}", i, flag);
            if flag && dp.get(&i.to_string()).is_some() {
                i += 1;
                continue;
            }
            let nums_count = get_number(&strs[i]);

            if !flag && m > n {
                if nums_count.1 > nums_count.0 {
                    i += 1;
                    if !flag && i == len {
                        flag = true;
                        i = 0;
                    }
                    continue;
                }
            }
            temp.0 += nums_count.0;
            temp.1 += nums_count.1;
            println!("--> iiiiiiii 】==> {:?} {:?} {:?}", i, temp, nums_count);
            if i == 0 {
                println!(" 01 01 01 01  {:?} {:?}", nums_count, temp);
            }
            // println!("【 nums_count 】==> {:?}", nums_count);
            if temp.0 <= m as usize && temp.1 <= n as usize {
                count += 1;
                dp.entry(i.to_string()).or_insert(nums_count);
            } else {
                temp.0 -= nums_count.0;
                temp.1 -= nums_count.1;
                if !flag {
                    flag = true;
                    i = 0;
                    continue;
                } else {
                    println!("【 iiiiiiii 】==> {:?}", i);
                    break;
                }
            }
            i += 1;
            println!("i == leni == leni == leni == leni == len {} {}", flag, i);
            if !flag && i == len {
                flag = true;
                i = 0;
                continue;
            }
        }
        println!("【 dp 】==> {:?}", dp);

        println!("【 count 】==> {:?}", count);

        if count == 1 {
            let result = Self::find_max_form(strs[1..len].to_vec(), m, n);
            count = count.max(result)
        }

        count
    }

    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let row = strs.len() + 1;
        let nn = (n + 1) as usize;
        let col = ((m + 1) * (nn as i32)) as usize;

        let mut pack_vol = vec![(0, 0); col];
        for i in 0..col {
            let val = decimal_to_quaternary_tuple(i, nn);
            pack_vol[i] = val
        }
        let mut dp = vec![vec![(0, 0, 0); col]; row];

        for r in 1..row {
            let line_val = get_number(&strs[r - 1]);
            // println!("【 nums 】==> {:?}", line_val);
            for l in 1..col {
                let pack_val = pack_vol[l];
                // println!("【 pack_val 】==> {:?}", pack_val);
                if line_val.0 <= pack_val.0 && line_val.1 <= pack_val.1 {
                    let pre = dp[r - 1][l];
                    // println!("【 pack_val 】==> {:?} {:?}", pack_val, line_val);
                    let idx = (pack_val.0 - line_val.0) * nn + (pack_val.1 - line_val.1);
                    let front = dp[r - 1][idx];

                    if (front.2 + 1) > pre.2 {
                        let zero = line_val.0 + front.0;
                        let one = line_val.1 + front.1;
                        dp[r][l] = (zero, one, front.2 + 1);
                        continue;
                    }
                }
                dp[r][l] = dp[r - 1][l];
            }
        }

        let result = dp.last().unwrap().last().unwrap();
        // println!("【 result 】==> {:?}", result);

        result.2
    }
}

fn get_number(s: &String) -> (usize, usize) {
    let zero = s.matches('0').count();
    let one = s.matches('1').count();

    (zero, one)
}

fn decimal_to_quaternary_tuple(decimal: usize, base: usize) -> (usize, usize) {
    let digit1 = decimal / base;
    let digit2 = decimal % base;
    (digit1, digit2)
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::{decimal_to_quaternary_tuple, Solution};

    #[test]
    fn should_work() {
        let result = Solution::find_max_form(
            vec![
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "1".to_string(),
            ],
            2,
            2,
        );
        assert_eq!(result, 4);
        println!("-------------------------");

        let result = Solution::find_max_form(
            vec![
                "111".to_string(),
                "1000".to_string(),
                "1000".to_string(),
                "1000".to_string(),
            ],
            9,
            3,
        );
        assert_eq!(result, 3);

        println!("-------------------------");
        let result = Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string(),
            ],
            5,
            3,
        );
        assert_eq!(result, 4);

        println!("-------------------------");
        let result = Solution::find_max_form(
            vec![
                "1100".to_string(),
                "100000".to_string(),
                "011111".to_string(),
            ],
            6,
            6,
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn decimal_to_quaternary_should_work() {
        let result = decimal_to_quaternary_tuple(4, 4);
        println!("【 result 】==> {:?}", result);
    }
}
