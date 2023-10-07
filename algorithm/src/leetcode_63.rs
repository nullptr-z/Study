pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() < 1 {
            return 0;
        };

        let r = obstacle_grid.len();
        let l = obstacle_grid[0].len();
        //         println!("<r,l>:{},{}", r, l);

        let (n, k) = n_k(r, l);

        let count = combination(n, k);
        //         println!("【 count 】==> {:?}", count);

        let mut roadblock = 0;
        for (nn, items) in obstacle_grid.iter().enumerate() {
            for (kk, value) in items.iter().enumerate() {
                if *value == 1 {
                    //                     println!("<m,n>:{},{}", nn + 1, kk + 1);
                    let (n, k) = n_k(nn + 1, kk + 1);
                    let f_count = combination(n, k);
                    let (n, k) = n_k(r - nn, l - kk);
                    let l_count = combination(n, k);
                    roadblock = f_count * l_count;
                    //                     println!("【 f_count 】==> {:?}", f_count);
                    //                     println!("【 l_count 】==> {:?}", l_count);
                }
            }
        }

        (count - roadblock) as i32
    }
}

fn n_k(r: usize, l: usize) -> (usize, usize) {
    let n = (r - 1) + (l - 1);
    let k = r - 1;

    (n, k)
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn combination(n: usize, k: usize) -> usize {
    //     println!("【 n 】==> {:?}", n);
    //     println!("【  k 】==> {:?}", k);
    if k > n {
        return 0;
    }
    factorial(n) / (factorial(k) * factorial(n - k))
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn unique_paths_with_obstacles_should_work() {
        let test_vec = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let result = Solution::unique_paths_with_obstacles(test_vec);
        //         println!("【 result 】==> {:?}", result);
    }
}
