use anyhow::{anyhow, Result};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Condvar, Mutex,
    },
};

// 生产者
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

// 消费者
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    cache: VecDeque<T>,
}

/**
	* queue      消费列队,生产者和消费者共享,用 Mutex互斥访问,用 Condvar通知
	* available  消费者挂起等待生产者通知进行唤醒
	* senders    生产者计数
	* receivers  消费者计数
	*/
struct Shared<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
    senders: AtomicUsize,
    receivers: AtomicUsize,
}

impl<T> Sender<T> {
    // 生产者写一个数据
    pub fn send(&mut self, t: T) -> Result<()> {
        if self.total_receive() == 0 {
            return Err(anyhow!("已无消费者"));
        }

        // 加锁,压入数据,释放锁
        let was_empty = {
            let mut inner = self.shared.queue.lock().unwrap();
            let empty = inner.is_empty();
            inner.push_back(t);
            empty
        };

        // 判断列队是否为空,为空时代表消费者线程可能已经全部被挂起,这时需要唤醒一个消费者
        if was_empty {
            // 通知一个被挂起的消费者线程,可以读数据了
            self.shared.available.notify_one()
        }

        Ok(())
    }

    pub fn total_receive(&self) -> usize {
        self.shared.receivers.load(Ordering::SeqCst)
    }

    pub fn total_queued_items(&self) -> usize {
        let queue = self.shared.queue.lock().unwrap();
        queue.len()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        self.shared.senders.fetch_add(1, Ordering::AcqRel);
        Self {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let old = self.shared.senders.fetch_sub(1, Ordering::AcqRel);
        // 如果生产者全部走掉了, 就唤醒消费者读数据,读不到就出错结束线程(释放)
        // 没有生产者了,消费者也就没意义,需要进行释放内存资源
        if old <= 1 {
            // 由于实现是 MPSC,  notify_all 和 notify_all 作用是一样的
            self.shared.available.notify_one()
        }
    }
}

impl<T> Receiver<T> {
    // 接受者读取一个数据
    pub fn recv(&mut self) -> Result<T> {
        // 取缓存中的值,无锁操作
        if let Some(t) = self.cache.pop_front() {
            return Ok(t);
        }

        // 拿到列队锁
        let mut inner = self.shared.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(t) => {
                    // 一次将共享列队中的数据全部去读到缓存
                    // 交换(swap)消费者的缓存列队和共享列队
                    // 从消费者缓存中读取就相当于无锁的 self.queue
                    if !inner.is_empty() {
                        std::mem::swap(&mut self.cache, &mut inner);
                    }
                    return Ok(t);
                }
                // 没有读到数据,并且已经不存在生产者了,返回错误释放锁
                None if self.total_senders() == 0 => return Err(anyhow!("已无发送者sender")),
                // 读不到数据,把锁交给 available Condvar, 它会释放锁并挂起线程, 等到 notify
                None => {
                    // Condvar等待挂起线程通知 notify, 再次获得锁并返回 锁MutexGuard;
                    // loop返回循环的开始回去拿数据, 这是为什么 Condvar 要在 loop 里使用
                    inner = self
                        .shared
                        .available
                        .wait(inner)
                        .map_err(|_| anyhow!("等待中.."))?;
                }
            }
        }
    }

    pub fn total_senders(&self) -> usize {
        self.shared.senders.load(Ordering::SeqCst)
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // 消费者离开时,消费者计数-1
        self.shared.receivers.fetch_sub(1, Ordering::SeqCst);
    }
}

impl<T> Iterator for Receiver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv().ok()
    }
}

/// 创建一个 unbounded channel
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Shared::default();
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared,
            cache: VecDeque::with_capacity(INITIAL_SIZE),
        },
    )
}

const INITIAL_SIZE: usize = 32;
impl<T> Default for Shared<T> {
    fn default() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(INITIAL_SIZE)),
            available: Condvar::new(),
            senders: AtomicUsize::new(1),
            receivers: AtomicUsize::new(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;
    // 此处省略所有 test case

    #[test]
    fn channel_should_work() {
        let (mut s, mut r) = unbounded();
        s.send("Hello World".to_string()).unwrap();
        let msg = r.recv().unwrap();
        assert_eq!(msg, "Hello World");
    }

    #[test]
    fn multiple_senders_should_work() {
        let (mut s, mut r) = unbounded();
        let mut s1 = s.clone();
        let mut s2 = s.clone();

        let t = thread::spawn(move || {
            s.send(1).unwrap();
        });
        let t1 = thread::spawn(move || {
            s1.send(2).unwrap();
        });
        let t2 = thread::spawn(move || {
            s2.send(3).unwrap();
        });
        for header in [t, t1, t2] {
            header.join().unwrap();
        }
        let mut result = [r.recv().unwrap(), r.recv().unwrap(), r.recv().unwrap()];
        result.sort();
        assert_eq!(result, [1, 2, 3]);
    }

    // 判断线程是否被阻塞,我们可以通过检测“线程是否退出”来间接判断
    #[test]
    fn receive_should_be_blocked_when_nothing_to_read() {
        let (mut s, r) = unbounded();
        let mut s1 = s.clone();
        thread::spawn(move || {
            for (idx, i) in r.into_iter().enumerate() {
                // 如果读到数据,确保他和发送的数据一致
                assert_eq!(idx, i);
            }
            // 读不到应该休眠，所以不会执行到这一句，执行到这一句说明逻辑出错
            assert!(false);
        });

        thread::spawn(move || {
            for i in 0..100usize {
                s.send(i).unwrap();
            }
        });
        // 1ms 足够让生产者发完 100 个消息，消费者消费完 100 个消息并阻塞
        thread::sleep(Duration::from_millis(1));
        for i in 100..200usize {
            s1.send(i).unwrap();
        }
        // 留点时间让 receiver 处理
        thread::sleep(Duration::from_millis(1));

        // 如果 receiver 被正常唤醒处理，那么队列里的数据会都被读完
        assert_eq!(s1.total_queued_items(), 0);
    }

    /**
     * 没有生产者了,消费者还尝试读取数据应该返回错误
     */
    #[test]
    fn last_senders_drop_should_error_when_receive() {
        let (s, mut r) = unbounded();
        let s1 = s.clone();
        let senders = [s, s1];
        let total = senders.len();

        for mut sender in senders {
            thread::spawn(move || {
                sender.send(1).unwrap();
            })
            .join()
            .unwrap();
        }

        for _ in 0..total {
            r.recv().unwrap();
        }

        assert!(r.recv().is_err());
    }

    /**
     * 没有消费者了错误返回
     */
    #[test]
    fn receive_drop_should_error_when_send() {
        let (mut s1, mut s2) = {
            let (s, _) = unbounded();
            let s2 = s.clone();
            (s, s2)
        };
        assert!(s1.send(1).is_err());
        assert!(s2.send(2).is_err());
    }

    /**
     * 如果 Receiver 被阻塞，而此刻所有 Sender 都走了，那么 Receiver 就没有人唤醒，会带来资源的泄露
     * Sender Drop 最后一个生产者释放时,唤醒所有消费者,消费者发现没有生产者了,错误返回
     */
    #[test]
    fn receive_shall_be_notified_when_all_senders_exit() {
        let (s, mut r) = unbounded::<usize>();
        /// 用于两个线程同步
        let (mut sender, mut receive) = unbounded::<usize>();
        let t1 = thread::spawn(move || {
            // 保证 r.recv() 先于 t2 的 drop 执行
            sender.send(1).unwrap();
            assert!(r.recv().is_err());
        });
        thread::spawn(move || {
            receive.recv().unwrap();
            drop(s);
        });
        t1.join().unwrap();
    }

    /**
     * 从无锁的消费者缓存中读取数据
     */
    #[test]
    fn channel_fast_path_should_work() {
        let (mut s, mut r) = unbounded();
        let mut msg_count: usize = 10;
        for i in 0..msg_count {
            s.send(i).unwrap();
        }

        assert!(r.cache.is_empty());
        // 读取第一个数据,会进行列队区交换
        assert_eq!(0, r.recv().unwrap());
        msg_count -= 1;
        // 此刻应该有{msg_count -1}个条数据在缓存中
        assert_eq!(msg_count, r.cache.len());
        // queue中应该没有数据
        assert_eq!(0, s.total_queued_items());
        // 从ache里读取剩下的数据
        for (idx, i) in r.take(msg_count).into_iter().enumerate() {
            assert_eq!(idx + 1, i);
        }
    }
}
