// 1个障碍<4,2>
// dp =
// [
//  [1, 1, 1, 1],
//  [1, 2, 3, 4],
//  [1, 3, 6, 10],
//  [1, 0, 6, 16],
//  [1, 1, 7, 23]
// ]

// 2个障碍<3,3>
// dp =
// [
//  [1, 1, 1, 1],
//  [1, 2, 3, 4],
//  [1, 3, 0, 4],
//  [1, 0, 0, 4],
//  [1, 1, 1, 5]
// ]

pub struct Solution;

impl Solution {
    /// 动态规划，这个版本可能更直观；类似帕斯卡恒等式求解方式
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() < 1 {
            return 0;
        };

        let n = obstacle_grid[0].len();
        // 保存的是上一行数据
        let mut dp = vec![0; n];
        dp[0] = 1;

        for row in obstacle_grid.iter() {
            // println!("【 dp 】==> {:?}", dp);
            for (n_idx, &val) in row.iter().enumerate() {
                if val == 1 {
                    // 1为障碍
                    dp[n_idx] = 0;
                } else if n_idx > 0 {
                    // 当前位置=当前行前一列 + 上一行同列； 效果等同于[r][l-1] + [r-1][l]
                    // dp[n_idx - 1]是同行前一列, dp[n_idx]就是上一行数据同列值
                    dp[n_idx] += dp[n_idx - 1];
                }
            }
        }

        println!("【 dp 】==> {:?}", dp);
        dp[n - 1]
    }

    /// 动态规划，这个版本可能更直观
    pub fn unique_paths_with_obstacles2(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() < 1 {
            return 0;
        };

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut arr = vec![vec![0; n + 1]; m + 1];
        arr[0][1] = 1;
        println!("【 arr 】==> {:?}", arr);

        for (m, items) in obstacle_grid.iter().enumerate() {
            for (n, val) in items.iter().enumerate() {
                let r = m + 1;
                let l = n + 1;
                if *val == 0 {
                    arr[r][l] = tt(&arr, r, l);
                } else {
                    arr[r][l] = 0
                }
            }
            print!("\n",);
        }

        show_arr(&arr);

        *arr.last().unwrap().last().unwrap()
    }

    /// 组合数版本，缺陷不适合多陷阱经情况，需要处理重叠部分
    pub fn for_combinatorics(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.len() < 1 {
            return 0;
        };

        let r = obstacle_grid.len();
        let l = obstacle_grid[0].len();
        println!("<r,l>:{},{}", r, l);

        let (n, k) = n_k(r, l);

        let count = combination(n, k);
        println!("【 count 】==> {:?}", count);

        let mut roadblock = 0;
        for (nn, items) in obstacle_grid.iter().enumerate() {
            for (kk, value) in items.iter().enumerate() {
                if *value == 1 {
                    println!("<m,n>:{},{}", nn + 1, kk + 1);
                    let (n, k) = n_k(nn + 1, kk + 1);
                    let f_count = combination(n, k);
                    let (n, k) = n_k(r - nn, l - kk);
                    let l_count = combination(n, k);
                    roadblock = f_count * l_count;
                    println!("【 f_count 】==> {:?}", f_count);
                    println!("【 l_count 】==> {:?}", l_count);
                }
            }
        }

        (count - roadblock) as i32
    }
}

fn tt(arr: &Vec<Vec<i32>>, r: usize, l: usize) -> i32 {
    arr[r][l - 1] + arr[r - 1][l]
}

fn show_arr(arr: &Vec<Vec<i32>>) {
    // 打印矩阵
    for items in arr[1..].iter() {
        for val in &items[1..] {
            print!("{} ", val);
        }
        println!();
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
    println!("【 n 】==> {:?}", n);
    println!("【  k 】==> {:?}", k);
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
        println!("【 result 】==> {:?}", result);
    }

    #[test]
    fn for_combinatorics_should_work() {
        let test_vec = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 0, 0, 0],
        ];
        let result = Solution::for_combinatorics(test_vec);
        println!("【 result 】==> {:?}", result);
    }
}
