impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut flag = 0;
        let len = digits.len() - 1;

        if digits[len] == 9 {
            flag = 1;
            digits[len] = 0
        } else {
            digits[len] += 1;
            return digits;
        }

        digits.reverse();
        for i in 1..digits.len() {
            let sum = digits[i] + flag;
            flag = 0;
            if sum > 9 {
                digits[i] = 0;
                flag = 1;
            } else {
                digits[i] = sum
            }
            if flag != 1 {
                break;
            }
        }

        if flag == 1 {
            digits.push(flag);
        }
        digits.reverse();

        println!("【 digits 】==> {:?}", digits);

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::plus_one(vec![1, 2, 9]);
    }
}

pub struct Solution;
