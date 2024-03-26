impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;

        let len = height.len();
        let mut l = vec![height[0]; len];
        let mut r = vec![height[len - 1]; len];

        for i in 1..len {
            l[i] = l[i - 1].max(height[i])
        }

        for i in (0..len - 1).rev() {
            r[i] = r[i + 1].max(height[i])
        }

        for i in 0..len {
            sum += l[i].min(r[i]) - height[i]
        }
        sum
    }
}

struct Solution;
