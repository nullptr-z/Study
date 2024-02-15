impl Solution {
    pub fn largest_rectangle_area(mut hs: Vec<i32>) -> i32 {
        hs.insert(0, 0);
        hs.push(0);

        let mut max = 0;
        let mut stack = vec![0; hs.len()];

        for (right, &val) in hs.iter().enumerate() {
            while !hs.is_empty() && val < hs[*stack.last().unwrap()] {
                let cur = stack.pop().unwrap();
                let left = stack.last().unwrap();
                let h = hs[cur];
                let w = (right - left - 1) as i32;
                max = max.max(h * w);
            }
            stack.push(right)
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        let ret = Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
