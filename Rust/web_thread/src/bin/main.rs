
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use web_thread::{ThreadPool};

fn main() {
    // 获取localhost:端口
    let server_url= format!("{}:7878",web_thread::get_ip().unwrap());
    // 监听/绑定TCP链接,非管理员只能监听大于1024的端口
    let listener=TcpListener::bind(&server_url).unwrap();
    println!("服务地址：http://{}", server_url);

    // 创建线程池：4个
    let pool = ThreadPool::new(4);

    // incoming:返回TcpStream类型的流
    // 流：包含了客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求/响应过程
    for tcpstream in listener.incoming().take(4) {
        let stream=tcpstream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }

    println!("服务停止运行！");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer=[0;512];     // 长度512缓冲区，初始化为0

    stream.read(&mut buffer).unwrap();  // 将流中链接信息写入缓冲区

    // 函数名的 “lossy” 部分来源于当其遇到无效的 UTF-8 序列时的 行为：它使用 � ， U+FFFD REPLACEMENT CHARACTER ，来代替无效序列
    // println!("Requst: {}", String::from_utf8_lossy(&buffer[..]));
    // println!("{}",String::from_utf8(buffer.to_vec()).expect("Found invalid UTF-8"));

    //# regin 这部分为业务逻辑，根据不同请求返回响应页面
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // 匹配对应的请求头；
    // starts_with: 将数组中开头部分与指定字符串进行匹配
    let (status, page_file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hiRust.html")
    } else if buffer.starts_with(sleep) {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hiRust.html")
    } else {
        ("HTTP/1.1 404 page not found\r\n\r\n", "404.html")
    };
    let contents = std::fs::read_to_string(page_file_name).unwrap();    // 读取`page_file_name`文件
    let response = format!("\n{}{}", status, contents);                      // 拼接响应状态 和 响应内容
    //# endregin

    stream.write(response.as_bytes()).unwrap();                                 // 将响应字符串写入流(发送)到链接
    stream.flush().unwrap();                                                        // 阻塞线程,保证流write写入完成
}
