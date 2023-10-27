impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut flag = 1;
        digits.reverse();
        for i in 0..digits.len() {
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
