//

pub struct Solution;

impl Solution {
    pub fn most_points(mut questions: Vec<Vec<i32>>) -> i64 {
        let len = questions.len();
        let mut dp = vec![0; questions.len()];
        let mut max: i32 = 0;

        let mut i = 0;
        while i < len {
            if questions[i][0] == 0 {
                i += 1;
                continue;
            }
            dp[i] = questions[i][0];

            let limit = questions[i][1] as usize;
            let next_idx = (i + limit + 1) as usize;
            if next_idx >= questions.len() {
                dp[i] = questions[i][0];
                max = max.max(dp[i]);
                println!("【 next_idx 】==> {:?} {}", next_idx, questions.len());
                i += 1;
                continue;
            }

            let next = questions[next_idx].clone();

            dp[i] += next[0];
            max = max.max(dp[i]);
            for j in i..next_idx {
                let t_j = &questions[j];
                if (j + t_j[1] as usize) == limit {
                    dp[i] = dp[i].max(t_j[0] + next[0]);
                    max = max.max(dp[i]);
                    if j - i == 1 {
                        i += 1;
                    } else {
                        questions[j][0] = 0;
                        questions[j][1] = 0;
                    }
                }
            }
            questions[next_idx][0] += dp[i];
            max = max.max(questions[next_idx][0]);
            if next_idx == questions.len() - 1 {
                break;
            }
            i += 1;
        }

        println!("【 questions 】==> {:?}", questions);
        println!("【 dp 】==> {:?}  {}", dp, max);

        max as i64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // Solution::most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]]);
        // Solution::most_points(vec![vec![43, 5]]);
        // Solution::most_points(vec![
        //     vec![12, 46],
        //     vec![78, 19],
        //     vec![63, 15],
        //     vec![79, 62],
        //     vec![13, 10],
        // ]);
        Solution::most_points(vec![
            vec![21, 5],
            vec![92, 3],
            vec![74, 2],
            vec![39, 4],
            vec![58, 2],
            vec![5, 5],
            vec![49, 4],
            vec![65, 3],
        ]);
    }
}
