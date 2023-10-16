//

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        // let mut f_ptr = 0;
        let mut temp = vec![0; (m + n) as usize];
        let mut f_ptr = 0;
        let mut l_ptr = 0;

        let mut i = 0;
        while i < temp.len() {
            let f_val = nums1[f_ptr];
            let l_val = {
                if l_ptr < (n as usize) {
                    nums2[l_ptr]
                } else {
                    i32::MAX
                }
            };
            if f_ptr < (m as usize) && f_val <= l_val {
                temp[i] = f_val;
                f_ptr += 1;
            } else if f_val > l_val || f_ptr == ((m) as usize) {
                temp[i] = l_val;
                l_ptr += 1;
            }
            i += 1;
        }
        println!("【 temp 】==> {:?}", temp);
        for (i, v) in temp.into_iter().enumerate() {
            nums1[i] = v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // Solution::merge(&mut vec![1, 2, 3, 0, 0, 0], 3, &mut vec![2, 5, 6], 3);
        Solution::merge(&mut vec![2, 0], 1, &mut vec![1], 1);
    }
}
