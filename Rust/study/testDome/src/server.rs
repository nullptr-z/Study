// use std::{
//     io::{Read, Write},
//     net::TcpListener,
// };

// fn main() {
//     let addr = "127.0.0.1:8787";

//     let server_socket = TcpListener::bind(addr).unwrap();

//     let mut stream = server_socket.accept().unwrap();

//     let mut buf = [0; 10];
//     stream.0.read(&mut buf);

//     print!("buf:{:?},stream:{:?}", buf, stream.1);
//     buf.reverse();

//     stream.0.write(&mut buf);

//     let a = 1;
// }

use std::net::UdpSocket;

fn main() {
    let addr = "127.0.0.1:8787";

    let server_socket = UdpSocket::bind(addr).unwrap();
    let mut buf = [0; 10];

    let stream_s = server_socket.recv(&mut buf).unwrap();

    let binding = buf.to_ascii_uppercase();

    let to_up = std::str::from_utf8(&binding).unwrap();
    print!("stream_s：{}，buf：{:?}", stream_s, to_up);

    server_socket.connect("127.0.0.1:8788").unwrap();
    server_socket.send(to_up.as_bytes());
}
