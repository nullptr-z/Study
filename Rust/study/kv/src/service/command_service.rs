use std::{borrow::BorrowMut, mem::swap, result};

use http::StatusCode;

use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut vec = Vec::with_capacity(self.keys.len());
        let table = &self.table;
        for key in self.keys.iter() {
            let cmd: Option<CommandResponse> = match store.get(table, key) {
                Ok(Some(v)) => {
                    vec.push(v);
                    None
                }
                Ok(None) => Some(KvError::NotFound(table.clone(), key.clone()).into()),
                Err(e) => Some(e.into()),
            };
            if cmd.is_some() {
                return cmd.unwrap();
            }
        }
        vec.into()
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute(mut self, store: &impl Storage) -> CommandResponse {
        let table = &self.table;
        let mut vec: Vec<Value> = Vec::with_capacity(self.pairs.len());
        // let mut pairs = Vec::with_capacity(self.pairs.len());
        // swap(&mut pairs, &mut self.pairs);
        for v in self.pairs.iter() {
            match store.set(table, v.key.clone(), v.clone().value.unwrap_or_default()) {
                Ok(None) => {}
                Ok(Some(v)) => {
                    vec.push(v);
                }
                Err(e) => return e.into(),
            };
        }
        vec.into()
    }
}

impl CommandService for Hdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.del(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmdel {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let table = &self.table;
        let mut vec: Vec<Value> = Vec::with_capacity(self.keys.len());
        for key in self.keys {
            match store.del(table, &key) {
                Ok(Some(v)) => {
                    vec.push(v);
                }
                Ok(None) => {}
                Err(e) => return e.into(),
            };
        }
        vec.into()
    }
}

impl CommandService for Hexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let table = &self.table;
        match store.contains(table, &self.key) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmexist {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut flags = false;
        let mut message: Vec<u8> = vec![b'0'; self.keys.len()];
        for (i, key) in self.keys.iter().enumerate() {
            let result = store.contains(&self.table, key);
            if result.is_ok() {
                if result.ok().unwrap() {
                    message[i] = b'1';
                } else {
                    flags = true;
                }
            } else {
                return result.err().unwrap().into();
            }
        }

        if flags {
            return CommandResponse {
                status: StatusCode::NO_CONTENT.as_u16() as _,
                message: String::from_utf8_lossy(message.as_slice()).to_string(),
                ..Default::default()
            };
        }

        CommandResponse {
            status: StatusCode::OK.as_u16() as _,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command_request::RequestData;

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hmget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10.into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hset("score", "u2", 20.into());
        dispatch(cmd, &store);

        let cmd = CommandRequest::new_hmget("score", vec!["u1".into(), "u2".into()]);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into(), 20.into()], &[]);
    }

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("source", "Hello", "world".into());
        dispatch(cmd.clone(), &store);
    }

    #[test]
    fn hmset_should_work() {
        let store = MemTable::new();
        let paris = &[
            Kvpair::new("Hello", "world".into()),
            Kvpair::new("哈喽", "世界".into()),
        ];

        // let cmd = CommandRequest::new_hmset("source", paris);
        // dispatch(cmd.clone(), &store);
        let cmd = CommandRequest::new_hmset("source", paris);
        let rep = dispatch(cmd.clone(), &store);

        assert_res_ok(rep, &[], &[]);
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let pairs = &[
            Kvpair::new("u1", 10.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 13.into()),
            Kvpair::new("u4", 2.into()),
        ];
        let cmds = pairs.iter().map(|item| {
            CommandRequest::new_hset("score", item.key.clone(), item.value.clone().unwrap())
        });

        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);

        assert_res_ok(res, &[], pairs);
    }

    #[test]
    fn hdel_should_work() {
        let store = MemTable::new();
        let paris = &[
            Kvpair::new("Hello", "world".into()),
            Kvpair::new("哈喽", "世界".into()),
        ];

        let cmd = CommandRequest::new_hmset("source", paris);
        dispatch(cmd.clone(), &store);

        let cmd = CommandRequest::new_hdel("source", "Hello");
        let rep = dispatch(cmd, &store);

        assert_res_ok(rep, &["world".into()], &[])
    }

    fn hmdel_should_work() {
        let store = MemTable::new();
        let paris = &[
            Kvpair::new("Hello", "world".into()),
            Kvpair::new("哈喽", "世界".into()),
        ];

        let cmd = CommandRequest::new_hmset("source", paris);
        dispatch(cmd.clone(), &store);

        let cmd = CommandRequest::new_hmdel("source", ["Hello".into(), "哈喽".into()]);
        let rep = dispatch(cmd, &store);
        assert_res_ok(rep, &["world".into(), "世界".into()], &[]);
    }

    #[test]
    fn hexist_shold_work() {
        let store = MemTable::new();
        let paris = &[
            Kvpair::new("Hello", "world".into()),
            Kvpair::new("哈喽", "世界".into()),
        ];

        let cmd = CommandRequest::new_hmset("source", paris);
        dispatch(cmd.clone(), &store);

        let cmd = CommandRequest::new_hexist("source", "哈喽");
        let rep = dispatch(cmd.clone(), &store);
        assert_res_ok(rep, &[], &[]);

        let cmd = CommandRequest::new_hexist("source", "hello");
        let rep = dispatch(cmd.clone(), &store);
        assert_res_err(rep, 204, "");
    }

    #[test]
    fn hmexist_shold_work() {
        let store = MemTable::new();
        let paris = &[
            Kvpair::new("Hello", "world".into()),
            Kvpair::new("哈喽", "世界".into()),
        ];

        let cmd = CommandRequest::new_hmset("source", paris);
        dispatch(cmd.clone(), &store);

        let cmd = CommandRequest::new_hmexist("source", ["Hello".into(), "哈喽".into()]);
        let rep = dispatch(cmd.clone(), &store);
        assert_res_ok(rep, &[], &[]);

        let cmd = CommandRequest::new_hmexist("source", ["hello".into(), "哈喽".into()]);
        let rep = dispatch(cmd.clone(), &store);
        assert_res_err(rep, 204, "01");
    }

    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hmget(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            RequestData::Hmset(v) => v.execute(store),
            RequestData::Hdel(v) => v.execute(store),
            RequestData::Hexist(v) => v.execute(store),
            RequestData::Hmexist(v) => v.execute(store),
            _ => todo!(),
        }
    }

    #[test]
    fn hget_with_non_exeit_key_should_return_404() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_err(res, 404, "Not found for table: score, key: u1");
    }

    // 测试成功返回的结果
    fn assert_res_ok(mut rep: CommandResponse, values: &[Value], paris: &[Kvpair]) {
        rep.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(rep.status, 200);
        assert_eq!(rep.message, "");
        assert_eq!(rep.values, values);
        assert_eq!(rep.pairs, paris);
    }

    // 测试失败返回的结果
    fn assert_res_err(mut rep: CommandResponse, code: u32, msg: &str) {
        rep.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(rep.status, code);
        assert!(rep.message.contains(msg));
        assert_eq!(rep.values, []);
        assert_eq!(rep.pairs, []);
    }
}
