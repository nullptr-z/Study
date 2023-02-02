use http::StatusCode;
use std::sync::Arc;
use tracing::{debug, info};

use crate::{command_request::RequestData, *};

mod command_service;

// Command抽象
pub trait CommandService {
    // 返回 response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

// Service 内部数据结构
pub struct ServiceInner<Store> {
    store: Store,
    /** 当服务器收到 CommandRequest请求时触发 */
    on_received: Vec<fn(&CommandRequest)>,
    /** 服务器处理完 CommandRequest,得到 CommandResponse时触发 */
    on_executed: Vec<fn(&CommandResponse)>,
    /** 在服务器发送 CommandResponse 之前触发。
     *  注意这个接口提供的是 &mut CommandResponse，这样事件的处理者可以根据需要，在发送前，修改 CommandResponse。
     *  */
    on_before_send: Vec<fn(&mut CommandResponse)>,
    /** 发送完成 CommandResponse后触发 */
    on_after_send: Vec<fn()>,
}

impl<Store: Storage> ServiceInner<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            on_received: Vec::new(),
            on_executed: Vec::new(),
            on_before_send: Vec::new(),
            on_after_send: Vec::new(),
        }
    }

    pub fn fn_received(mut self, f: fn(&CommandRequest)) -> Self {
        self.on_received.push(f);
        self
    }

    pub fn fn_executed(mut self, f: fn(&CommandResponse)) -> Self {
        self.on_executed.push(f);
        self
    }

    pub fn fn_before_send(mut self, f: fn(&mut CommandResponse)) -> Self {
        self.on_before_send.push(f);
        self
    }

    pub fn fn_after_send(mut self, f: fn()) -> Self {
        self.on_after_send.push(f);
        self
    }
}

impl<Store: Storage> From<ServiceInner<Store>> for Service<Store> {
    fn from(inner: ServiceInner<Store>) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner::new(store)),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        debug!("Got`得到 request: {:?}", cmd);
        self.inner.on_received.notify(&cmd);
        let mut res = dispatch(cmd, &self.inner.store);
        self.inner.on_executed.notify(&res);

        self.inner.on_before_send.notify(&mut res);
        if !self.inner.on_before_send.is_empty() {
            debug!("Executed`执行 response: {:?}", res);
        }

        res
    }
}

// 事件通知(不可变)
pub trait Notify<Arg> {
    fn notify(&self, arg: &Arg);
}

impl<Arg> Notify<Arg> for Vec<fn(&Arg)> {
    #[inline]
    fn notify(&self, arg: &Arg) {
        for f in self {
            f(arg)
        }
    }
}

// 事件通知(可变)
pub trait MutNotify<Arg> {
    fn notify(&self, arg: &mut Arg);
}

impl<Arg> MutNotify<Arg> for Vec<fn(&mut Arg)> {
    #[inline]
    fn notify(&self, arg: &mut Arg) {
        for f in self {
            f(arg)
        }
    }
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

// 从 Request中得到 Response
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        Some(RequestData::Hgetall(param)) => param.execute(store),
        Some(RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("Request has`有 no`否定 data".into()).into(),
        _ => KvError::Internal("Not`还没有 implemented`实施".into()).into(),
    }
}

// 测试返回成功的结果
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}

#[cfg(test)]
mod test {

    use std::thread;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_works() {
        // 我们需要一个 service结构至少包含 Storage
        // let service = Service::new(MemTable::default());
        let service: Service = ServiceInner::new(MemTable::default()).into();

        // service可以在多线程环境下运行，它的 clone应该是轻量级的
        let cloned = service.clone();

        // 创建一个多线程，在 table t1中写入 k1，v1
        let handel = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "key", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handel.join().unwrap();

        // 在当前线程下读取 table t1的 k1，应该返回 v1
        let res = service.execute(CommandRequest::new_hget("t1", "key"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::{Kvpair, Value};

#[test]
fn event_registration_should_work() {
    fn b(req: &CommandRequest) {
        info!("Got {:?}", req);
    }

    fn c(res: &CommandResponse) {
        info!("Got {:?}", res);
    }

    fn d(res: &mut CommandResponse) {
        res.status = StatusCode::CREATED.as_u16() as _;
    }

    fn e() {
        info!("Data is sent`数据已发送");
    }

    let service: Service = ServiceInner::new(MemTable::default())
        .fn_received(|_: &CommandRequest| {})
        .fn_received(b)
        .fn_executed(c)
        .fn_before_send(d)
        .fn_after_send(e)
        .into();

    let res = service.execute(CommandRequest::new_hset("t1", "kv1", "v1".into()));
    assert_eq!(res.status, StatusCode::CREATED.as_u16() as _);
    assert_eq!(res.message, "");
    assert_eq!(res.values, vec![Value::default()]);
}
