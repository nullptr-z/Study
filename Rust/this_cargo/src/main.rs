#![allow(non_snake_case)]
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(">>>>>>>返回首单词<<<<<<");
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("读取输入行错误");
    println!("字符串: {}", str);
    let fistWordLen = get_first_word(str);
    println!("首单词下标: {}", fistWordLen);

    let fistWordLen = "aa";
    println!("首单词下标: {}", fistWordLen);

    println!(">>>>>>>猜0-100之间的数字<<<<<<");
    let randNumber = rand::thread_rng().gen_range(0, 101);
    println!("已生成随机数0-100: {}", randNumber);
    loop {
        println!("请输入猜测的数字");
        let mut inValue = String::new();
        io::stdin().read_line(&mut inValue).expect("读取输入行错误");
        let inValue: u32 = match inValue.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("请输入合法数字");
                continue;
            }
        };
        println!("输入:{}", &inValue);

        match inValue.cmp(&randNumber) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("对了");
                break;
            }
        }
    }
}

fn get_first_word(str: String) -> usize {
    let strArr = str.as_bytes();
    for (i, &item) in strArr.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
