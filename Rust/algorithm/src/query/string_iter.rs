use std::collections::HashMap;

fn get_sub_string_max_length(s: String) -> i32 {
    let len = s.len();
    let mut str_to_chars = s.as_bytes();
    let mut max_len = 0;
    let mut ch_case = HashMap::new();

    for k in 0..len {
        ch_case.insert(str_to_chars[k], usize::MAX);
    }

    let mut left_ptr = 0; // 被比较位
    let mut right_ptr = 0; // 比较位置
    while max_len < (len - left_ptr) {
        let mut count = right_ptr - left_ptr;

        for j in right_ptr..len {
            let get_ch = ch_case.get_mut(&str_to_chars[j]).unwrap();
            match *get_ch == usize::MAX {
                true => {
                    *ch_case.get_mut(&str_to_chars[j]).unwrap() = j;
                    count += 1;
                }
                false => {
                    let temp = left_ptr; // 本轮循环起点
                    let temp_1 = *get_ch; // 冲突字符的起点
                    left_ptr = *get_ch + 1; // 下一轮循环的起点、冲突字符位置向后一步，跳过冲突字符
                    right_ptr = j + 1; //  发生冲突的位置，已经比较过了，向后移一步
                    *get_ch = j; // 冲突字符的位置更新
                    for k in temp..temp_1 {
                        // 将冲突位置之前的全部重置，剩下的都是经过比较，无冲突的
                        *ch_case.get_mut(&str_to_chars[k]).unwrap() = usize::MAX;
                    }
                    break;
                }
            };
        }

        max_len = max_len.max(count);
    }

    max_len as i32
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len: i32 = 0;
    let mut start: i32 = 0;
    let mut cache = [0; 128];
    for (i, ch) in s.chars().enumerate() {
        let index = ch as usize;
        start = start.max(cache[index]);
        max_len = max_len.max(i as i32 - start + 1);
        cache[index] = i as i32 + 1;
    }
    max_len
}

fn get_sub_string_max_length_quicks(s: String) -> i32 {
    let mut str_u8 = s.as_bytes();
    let len = str_u8.len();
    let mut cace: HashMap<u8, usize> = HashMap::new();

    let mut left_ptr = 0;
    let mut max_len = 0;

    for i in 0..len {
        let ch = cace.get(&str_u8[i]);
        let offset = i + 1;
        match ch {
            Some(_) => {
                left_ptr = left_ptr.max(*ch.unwrap());
                *cace.get_mut(&str_u8[i]).unwrap() = offset;
            }
            None => {
                cace.insert(str_u8[i], offset);
            }
        }
        // 后指针和起点的距离
        max_len = max_len.max(offset - left_ptr);
    }

    max_len as i32
}

#[test]
fn string_iter() {
    let str = "tmmzuxt";
    // let str = "tmmzuxtkxcvhg";
    let result = get_sub_string_max_length(str.to_string());
    assert_eq!(5, result);

    let str = "tmmzuxtkxcvhg";
    let result = get_sub_string_max_length_quicks(str.to_string());
    assert_eq!(7, result);

    let str = "tmmzuxtkxcvhg";
    let result = length_of_longest_substring(str.to_string());
    assert_eq!(7, result);
}
