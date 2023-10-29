// 段落最大和
pub fn max_nums_segment(array: Vec<i32>) -> i32 {
    if array.is_empty() {
        return 0; // 如果数组为空，返回0
    }

    let mut max = array[0]; // 将 max 初始化为数组的第一个元素
    let mut sum = array[0]; // 将 sum 初始化为数组的第一个元素

    for item in &array[1..] {
        sum = sum.max(0) + item; // 更新 sum，如果 sum 变为负数，则从当前元素重新开始
        max = max.max(sum); // 更新 max
    }

    max
}

#[cfg(test)]
mod tests {

    use crate::{array_utils::max_nums_segment, tree_utils::arrayToTree};

    #[test]
    fn should_work() {
        let result = max_nums_segment(vec![-1, 2]);
        println!("【 result 】==> {:?}", result);
        let result = max_nums_segment(vec![-3]);
        println!("【 result 】==> {:?}", result);
    }
}
