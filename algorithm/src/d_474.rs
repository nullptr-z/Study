use std::collections::HashMap;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let list = get_list(strs);
        let mut dp = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];

        for li in list.iter() {
            let zero_count = li.0 as usize;
            let one_count = li.1 as usize;
            // 可以理解成cap1,cap2两个背包，或者说两个维度的背包
            for z in (zero_count..=m as usize).rev() {
                for o in (one_count..=n as usize).rev() {
                    dp[z][o] = dp[z][o].max(dp[z - zero_count][o - one_count] + 1);
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }

    pub fn find_max_forms(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let list = get_list(strs);
        let mut dp: HashMap<(usize, usize), usize> = HashMap::new();
        dp.insert((0, 0), 0);

        for li in list.iter() {
            let zero_count = li.0 as usize;
            let one_count = li.1 as usize;

            let mut new_dp: HashMap<(usize, usize), usize> = dp.clone();

            for (z, o) in dp.keys() {
                let new_z = z + zero_count;
                let new_o = o + one_count;

                if new_z <= m as usize && new_o <= n as usize {
                    *new_dp.entry((new_z, new_o)).or_insert(0) =
                        (*new_dp.get(&(new_z, new_o)).unwrap_or(&0))
                            .max(*dp.get(&(*z, *o)).unwrap_or(&0) + 1);
                }
            }

            dp = new_dp;
        }

        *dp.values().max().unwrap_or(&0) as i32
    }
}

fn get_list(strs: Vec<String>) -> Vec<(i32, i32)> {
    let list = strs
        .iter()
        .map(|m| {
            m.chars().fold((0, 0), |(zero, one), char| match char {
                '0' => (zero + 1, one),
                '1' => (zero, one + 1),
                _ => (zero, one),
            })
        })
        .collect();
    list
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let r = Solution::find_max_form(
            ["10", "0001", "111001", "1", "0"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
            5,
            3,
        );
        println!("【 r 】==> {:?}", r);
    }
}

pub struct Solution;
