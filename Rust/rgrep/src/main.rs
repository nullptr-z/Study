use std::{env, fs};

struct Param {
    value: String,
    fileName: String,
}

impl Param {
    pub fn new(args: &Vec<String>) -> Result<Param, String> {
        if args.len() < 2 {
            return Err(String::from("需要提供至少2个参数，寻找的字符和查找的文件"));
        }

        Ok(Param {
            value: args[1].clone(),
            fileName: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let param = Param::new(&args[1..].to_vec());
    // let file=fs::read(path)

    println!("args: {:?}", args);
}
