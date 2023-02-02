use std::{
  net::UdpSocket,
  sync::{mpsc, Arc, Mutex},
  thread::{spawn, JoinHandle},
};

/** 获取本机IP */
pub fn get_ip() -> Option<String> {
  let socket = match UdpSocket::bind("0.0.0.0:0") {
    Ok(s) => s,
    Err(_) => return None,
  };

  match socket.connect("8.8.8.8:80") {
    Ok(()) => (),
    Err(_) => return None,
  };

  match socket.local_addr() {
    Ok(addr) => return Some(addr.ip().to_string()),
    Err(_) => return None,
  };
}
