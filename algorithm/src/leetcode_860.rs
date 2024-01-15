impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);

        for money in bills {
            let (mut ten_back, mut five_back) = (0, 0);
            if money == 5 {
                five += 1;
                continue;
            } else if money == 10 {
                ten += 1;
                five_back += 1;
            } else {
                ten_back += 1;
                five_back += 1;
            }

            if ten >= ten_back {
                ten -= ten_back
            } else {
                ten = 0;
                let diff = ten_back - ten;
                five_back += diff * 2; // 换成5元需要找零的张数
            }

            if five >= five_back {
                five -= five_back
            } else {
                return false;
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
        let ret = Solution::lemonade_change(vec![5, 5, 5, 10, 20]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
