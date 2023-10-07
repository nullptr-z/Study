mod sort;
use sort::*;
use std::time::Instant;
fn main() {
    let MAX = 10000usize;
    let mergeArray = get_gen_range_array(MAX, |num| {
        Message::new(num, format!("wawa-{}", num).as_str())
        // num
    });

    let merge = MergeSort::new(mergeArray);

    let now = Instant::now();
    let temp = merge.merge_sort();
    let consume_time_micros = now.elapsed().as_micros();
    let consume_time_millis = now.elapsed().as_millis();

    println!("长度:{}归并排序,耗时:{}毫秒", MAX, consume_time_millis);
    for (index, item) in temp.unwrap().iter().enumerate() {
        assert_eq!(item.num, index);
        // assert_eq!(*item, index);
    }

    assert_eq!(0, 0);
}
