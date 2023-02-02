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

/**
 * 在任何线 程收到消息之前，就尝试 join worker 0 了。
 * worker 0 还没有收到终止消息，所以主线程阻塞 直到 worker 0 结束。
 * 与此同时，每一个线程都收到了终止消息。
 * 一旦 worker 0 结束，主线程 就等待其他 worker 结束，此时他们都已经收到终止消息并能够停止了。
 *
 */

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job), // 新任务消息
  Terminate,   // 任务终止消息
}

struct Worker {
  id: u32,
  thread: Option<JoinHandle<()>>,
}

impl Worker {
  fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("线程{}接收到任务，开始执行！", id);
          job();
        }
        Message::Terminate => {
          println!("线程{}接受到终止信号，终止执行！", id);
          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
  /**
   * 创建线程池，建立通道(充当任务列队)
   *
   * @param size 线程池数量, 0时会panic
   * @return
   */
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    // 通道 receiver是单消费者的,(单接收端)
    // Arc智能指针,以引用计数来保证所有权机制在多引用情况下运行正常
    // Arc<Mutex<T>>: Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次 只有一个 worker 能从接收端得到任务
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers: Vec<Worker> = Vec::with_capacity(size);

    for id in 0..4 {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { sender, workers }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f); // Box封装 f类型：dyn FnOnce() + Send + 'static的闭包
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("子线程{}，本次任务执行结束！", worker.id);

      if let Some(thread) = worker.thread.take() {
        // Option 上的 take 方法会取出 Some 而留下 None
        thread.join().unwrap(); // 阻塞关联的线程，直至本次线程任务结束
      }
    }
  }
}
