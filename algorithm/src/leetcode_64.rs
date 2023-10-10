//

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mm = grid.len();
        let nn = grid[0].len();

        let mut temp = vec![vec![0; nn]; mm];
        let mut m_idx = (mm - 1) as i32;
        let mut n_idx = (nn - 1) as i32;

        while m_idx >= 0 {
            let m = m_idx as usize;
            while n_idx >= 0 {
                let n = n_idx as usize;
                let mut min = 0;
                let r_val = temp[m].get(n + 1);
                let d_val = {
                    if (m + 1) < mm {
                        let v = temp[m + 1][n];
                        if r_val.is_some() {
                            min = (*r_val.unwrap()).min(v);
                        } else {
                            min = v;
                        }

                        Some(v)
                    } else {
                        if r_val.is_some() {
                            min = *r_val.unwrap();
                        } else {
                            min = 0;
                        }

                        None
                    }
                };

                temp[m][n] = min + grid[m][n];

                n_idx -= 1;
            }
            n_idx = (nn - 1) as i32;
            m_idx -= 1;
        }

        let min_path = temp[0][0];
        // println!("【 temp 】==> {:?}", temp);
        // println!("【 min_path 】==> {:?}", min_path);

        min_path
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]);
    }
}
