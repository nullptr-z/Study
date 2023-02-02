use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::{
  easy::HighlightLines,
  highlighting::ThemeSet,
  parsing::SyntaxSet,
  util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

#[derive(Parser, Debug)] // 生成 parse() 函数
#[clap(version = "1.0", author = "Tyr Chen <tyr@chen.com>")]
struct Opts {
  #[clap(subcommand)]
  subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
  Get(Get),
  Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
  url: String,
}

#[derive(Parser, Debug)]
struct Post {
  #[clap(parse(try_from_str=parse_url))] // 验证url
  url: String,
  #[clap(parse(try_from_str=parse_kv_pair))] // 验证body格式
  body: Vec<KvPair>,
}

#[derive(Parser, Debug)]
struct KvPair {
  k: String,
  v: String,
}

impl FromStr for KvPair {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // 使用 =进行 split分拨，得到一个迭代器
    let mut split = s.split('=');
    let err = || anyhow!(format!("KvPair FromStr parse 错误: {}", s));
    Ok(Self {
      // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
      // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
      k: (split.next().ok_or_else(err)?).to_string(),
      // 从迭代器中取第二个结果作为 value
      v: (split.next().ok_or_else(err)?).to_string(),
    })
  }
}

fn parse_url(s: &str) -> Result<String> {
  let url: Url = s.parse()?;
  Ok(url.into())
}

// 因为我们为 KvPair实现了 FromStr，这里可以直接 s.parse()将字符串转换为 KvPair结构
fn parse_kv_pair(s: &str) -> Result<KvPair> {
  s.parse()
}

async fn get(client: Client, args: &Get) -> Result<()> {
  let resp = client.get(&args.url).send().await?;
  println!("{:?}", resp);
  Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
  let mut body = HashMap::new();
  for pair in args.body.iter() {
    body.insert(&pair.k, &pair.v);
  }
  let resp = client.post(&args.url).json(&body).send().await?;
  Ok(print_resp(resp).await?)
}

//#region 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
  print_status(&resp);
  print_headers(&resp);
  let mime = get_content_type(&resp);
  let body = resp.text().await?;
  print_body(mime, &body);
  Ok(())
}

//#region 打印 response
// 打印服务器返回的 HTTP status
fn print_status(resp: &Response) {
  let status = format!("{:?} {}", resp.version(), resp.status()).blue();
  println!("{}\n", status);
}
// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
  for (key, value) in resp.headers() {
    println!("{}: {:?}", key.to_string().green(), value);
  }
  print!("\n");
}

fn print_syntect(s: &str, ext: &str) {
  let ps = SyntaxSet::load_defaults_newlines();
  let ts = ThemeSet::load_defaults();
  let syntax = ps.find_syntax_by_extension(ext).unwrap();
  let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
  for line in LinesWithEndings::from(s) {
    let ranges = h.highlight(line, &ps);
    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
    println!("{}", escaped);
  }
}

// 打印服务器返回的 HTTP body
async fn print_body(m: Option<Mime>, body: &String) {
  match m {
    Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "body"),
    Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),
    _ => println!("{:?}", body),
  }
}
// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
  resp
    .headers()
    .get(header::CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().parse().unwrap())
}

//#endregion

#[tokio::main]
async fn main() -> Result<()> {
  let opts: Opts = Opts::parse();
  let mut headers = header::HeaderMap::new();
  // 为的 HTTP 客户端添加一些`缺省`的 HTTP 头
  headers.insert("X-POWERED-BY", "Rust".parse()?);
  headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
  let client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;

  let result = match opts.subcmd {
    SubCommand::Get(ref args) => get(client, args).await?,
    SubCommand::Post(ref args) => post(client, args).await?,
  };

  Ok(result)
}
