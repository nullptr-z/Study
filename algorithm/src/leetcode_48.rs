//

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut m = matrix.len();

        let mut count = 0;

        // 逐层处理
        while (m - count) >= 2 {
            let mut j = count;

            // 不同层级需要处理的列数不同
            while j < m - count - 1 {
                let mut target_row = 0;
                let mut target_col = 0;

                let mut i = 0;
                let mut row = count;
                let mut col = j;
                let mut temp = matrix[row][col];
                // 每次旋转交换4个位置
                while i < 4 {
                    target_row = col;
                    target_col = (m - 1) - (row);

                    let target_val = matrix[target_row][target_col];
                    matrix[target_row][target_col] = temp;
                    temp = target_val;

                    row = target_row;
                    col = target_col;

                    i += 1;
                }
                j += 1;
            }

            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let mut arr = vec![[1, 2, 3].into(), vec![4, 5, 6], vec![7, 8, 9]];
        // Solution::rotate(&mut arr);

        let mut arr = vec![
            [5, 1, 9, 11].into(),
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            [15, 14, 12, 16].into(),
        ];
        Solution::rotate(&mut arr);
        println!("【 arr 】==> {:?}", arr);
    }
}
