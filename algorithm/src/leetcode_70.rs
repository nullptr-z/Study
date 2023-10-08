//

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        let len = (n + 1) as usize;
        let mut arr = vec![1; len];
        arr[0] = 1;
        arr[1] = 1;
        for i in 2..len {
            arr[i] = arr[i - 1] + arr[i - 2];
        }

        *arr.last().unwrap()
    }

    // 第 N 个泰波那契数
    pub fn tribonacci(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        } else if n <= 2 {
            return 1;
        }

        let len = (n) as usize;
        let mut arr = vec![1; len];
        arr[0] = 1;
        arr[1] = 1;
        arr[2] = 2;
        for i in 3..len {
            arr[i] = arr[i - 1] + arr[i - 2] + arr[i - 3];
        }
        println!("【 arr 】==> {:?}", arr);

        *arr.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::climb_stairs(4);
    }

    #[test]
    fn tribonacci_should_work() {
        Solution::tribonacci(4);
    }
}
