impl Solution {
    // 结构为Vev[ Vec[ h, k ] ]
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 按身高排序
        people.sort_by(|f, b| {
            if f[0] == b[0] {
                // 基于h排序的情况下(h相等)，把k小的排在前面
                return f[1].cmp(&b[1]);
            }
            b[0].cmp(&f[0])
        });

        let mut result = vec![];
        for items in people {
            // 经过排序后，k就是自己的相对于前面所有所有的优先级别
            result.insert(items[1] as usize, items);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::reconstruct_queue(vec![
            [7, 0].to_vec(),
            [4, 4].to_vec(),
            [7, 1].to_vec(),
            [5, 0].to_vec(),
            [6, 1].to_vec(),
            [5, 2].to_vec(),
        ]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
