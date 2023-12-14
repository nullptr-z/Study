impl Solution {
    // `count`是遍历次数，初始为`n-1`
    /// 每当次数为0，改变高度
    /// 左右遍历结束，遍历次数减`n -= 1`并且恢复次数，上下遍历恢复次数为n
    pub fn generate_matrix(mut n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; n as usize]; n as usize];

        let mut direct: Direct = Direct::Right;

        let mut row = 0 as usize;
        let mut col = 0 as usize;
        let mut count = n - 1;

        for i in 0..(n * n) as i32 {
            matrix[row][col] = i + 1;
            count -= 1;

            let is_change = count == 0;
            match direct {
                Direct::Top => {
                    row -= 1;
                    if is_change {
                        direct = Direct::Right;
                        count = n;
                    }
                }
                Direct::Bottom => {
                    row += 1;
                    if is_change {
                        direct = Direct::Left;
                        count = n;
                    }
                }
                Direct::Right => {
                    col += 1;
                    if is_change {
                        n -= 1;
                        count = n;
                        direct = Direct::Bottom;
                    }
                }
                Direct::Left => {
                    col -= 1;
                    if is_change {
                        n -= 1;
                        count = n;
                        direct = Direct::Top;
                    }
                }
            }
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
