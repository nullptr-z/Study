impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        heap_sort(&mut nums);
        println!("【 nums 】==> {:?}", nums);

        nums[nums.len() - (k as usize)]
    }

    pub fn find_kth_largest_sort(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_by(|p, b| b.cmp(p));

        nums[(k - 1) as usize]
    }
}

fn heap_sort(arr: &mut Vec<i32>) {
    let len = arr.len();
    // 建堆
    for i in (0..(len / 2)).rev() {
        // 将i个元素置位
        heapify(arr, len, i)
    }

    // 排序
    for i in (0..len).rev() {
        // 将堆顶arr[0]和最后一个元素arr[i]交换，这样堆顶这个元素为有序的了
        let temp = arr[0];
        arr[0] = arr[i];
        arr[i] = temp;
        heapify(arr, i, 0);
    }
}

/**
 * 维护堆的性质
 * @param arr 存储堆的数组
 * @param len 数组的长度
 * @param i 待维护节点的下标
 */
fn heapify(arr: &mut Vec<i32>, len: usize, i: usize) {
    let mut root = i;
    let l = i * 2 + 1;
    let r = i * 2 + 2;

    // 找出他们最大的放到root
    if l < len && arr[l] > arr[root] {
        root = l;
    }
    if r < len && arr[r] > arr[root] {
        root = r;
    }
    if root != i {
        arr[root] ^= arr[i];
        arr[i] ^= arr[root];
        arr[root] ^= arr[i];
        heapify(arr, len, root);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::find_kth_largest(vec![1], 1);
    }
}

pub struct Solution;
