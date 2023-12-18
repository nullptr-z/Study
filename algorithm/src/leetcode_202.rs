use std::collections::HashSet;

impl Solution {
    /// hash法
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut sum = n;

        while sum != 1 {
            let mut nn = sum;
            let mut sum1 = 0;
            while nn != 0 {
                let b = nn % 10;
                sum1 += b * b;
                nn = nn / 10;
            }

            if set.get(&sum).is_some() {
                return false;
            }
            set.insert(sum);
            sum = sum1;
        }

        true
    }

    /// 双指针法
    pub fn is_happy_double_ptr(n: i32) -> bool {
        let get_sum = |mut nn: i32| -> i32 {
            let mut sum = 0;
            while nn != 0 {
                let b = nn % 10;
                sum += b * b;
                nn = nn / 10;
            }

            return sum;
        };

        let mut fast = get_sum(n);
        let mut slow = n;
        while fast != 1 {
            fast = get_sum(get_sum(fast));
            slow = get_sum(slow);
            if fast == slow {
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
        let ret = Solution::is_happy_double_ptr(13);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
