use std::{env, process};

use min_grep::Config;

fn main() {
    read_file()
}

fn read_file() {
    let args: Vec<String> = env::args().collect();
    println!("命令行参数： {:?}:", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("parse_config函数调用发生错误: {}", err);
        process::exit(1);
    });
    println!("查找文件:{} 查找内容: {}", config.filename, config.querystr);

    if let Err(e) = min_grep::run(config) {
        eprintln!("文件查找失败！:{}", e);
        process::exit(1);
    }
}
