use std::ops::{Add, Sub};

use crate::Max;

fn get_max_combination_sum<T: PartialOrd + Add<Output = T> + Sub<Output = T> + Clone + Copy>(
    arr: Vec<T>,
) -> Option<Vec<T>> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    if len == 1 {
        return Some(arr);
    }

    let mut dp: Vec<T> = Vec::with_capacity(len);
    let start_index = match arr[0] > arr[len - 1] {
        true => 0,
        false => 1,
    };

    let mut max: T;
    dp.push(arr[start_index]);
    dp.push(Max(arr[start_index], arr[start_index + 1]));

    for i in (start_index + 2)..len {
        max = arr[i] + dp[i - 2 - start_index];
        dp.push(Max(max, dp[i - 1 - start_index]));
    }

    Some(dp)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::create_user_list;

    use super::get_max_combination_sum;

    #[test]
    fn crete_dyc_programing() {
        // let arr = create_user_list(6, 1..10);
        let arr = [3, 1, 7, 10, 12];

        let mut result = get_max_combination_sum(arr.to_vec()).unwrap();

        // assert_ne!(result.pop().unwrap().age, 0);
        assert_ne!(result.pop().unwrap(), 0);
    }
}
