use std::fs;

fn main() {
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() < 3 {
        return;
    }

    let url = arg[1].to_string();
    let output = arg[2].to_string();

    println!("获取 HTML url:{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("转为 html 为 markdown..");

    let md = html2md::parse_html(&body);
    fs::write(&output, md.as_bytes()).unwrap();
    println!("转换为markdown，保存到文件{}", &output);
}
