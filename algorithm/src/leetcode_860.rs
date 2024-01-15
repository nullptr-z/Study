impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let (mut five, mut ten) = (0, 0);

        for money in bills {
            let ten_back = (money - 5) / 10;
            let is_five = (money - 5) % 10;
            let mut five_back3 = if is_five > 0 { 1 } else { 0 };

            if ten >= ten_back {
                ten -= ten_back
            } else {
                ten = 0;
                let diff = ten_back - ten;
                five_back3 += diff * 2; // 换成5元需要找零的张数
            }

            if five >= five_back3 {
                five -= five_back3
            } else {
                return false;
            }

            if money == 10 {
                ten += 1;
            } else if money == 5 {
                five += 1;
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
