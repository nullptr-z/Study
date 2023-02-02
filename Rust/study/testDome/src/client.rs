// use std::{
//     io::{Read, Write},
//     net::TcpStream,
// };

// fn main() {
//     let addr = "127.0.0.1:8787";

//     let mut client_socket = TcpStream::connect(addr).unwrap();

//     let stream = client_socket.write(&[1, 2, 3, 9, 9, 9]).unwrap();

//     let mut buf = [0; 10];

//     client_socket.read(&mut buf);

//     println!("buf:{:?}", buf);
// }

use std::net::UdpSocket;

fn main() {
    let addr = "127.0.0.1:8788";
    let client_socket = UdpSocket::bind(addr).unwrap();
    client_socket.connect("127.0.0.1:8787").unwrap();

    let stream = client_socket.send("hello".as_bytes()).unwrap();
    println!("stream: {}", stream);

    let mut buf = [0; 10];
    let rsp = client_socket.recv(&mut buf).unwrap();
    print!("rsp: {},buf:{:?}", rsp, std::str::from_utf8(&buf));
}
