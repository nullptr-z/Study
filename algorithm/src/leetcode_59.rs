impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let m = n as usize;
        let mut matrix = vec![vec![0; m]; m];

        let mut direct: Direct = Direct::Right;

        let mut r = (0) as usize;
        let mut l = (0) as usize;
        let mut n = n;
        let mut count = n;
        let mut i = 0;
        while i < (m * m) as i32 && n > 0 {
            matrix[r][l] = i + 1;
            count -= 1;

            let is_change = count == 0;
            if is_change {
                match direct {
                    Direct::Top => {
                        direct = Direct::Right;
                        count = n;
                    }
                    Direct::Bottom => {
                        direct = Direct::Left;
                        count = n;
                    }
                    Direct::Right => {
                        n -= 1;
                        count = n;
                        direct = Direct::Bottom;
                    }
                    Direct::Left => {
                        n -= 1;
                        count = n;
                        direct = Direct::Top;
                    }
                }
            }

            match direct {
                Direct::Top => {
                    r -= 1;
                }
                Direct::Bottom => {
                    r += 1;
                }
                Direct::Right => {
                    l += 1;
                }
                Direct::Left => {
                    l -= 1;
                }
            }

            i += 1;
        }

        matrix
    }
}

enum Direct {
    Top,
    Bottom,
    Right,
    Left,
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::generate_matrix(4);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
