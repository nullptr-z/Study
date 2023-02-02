use crate::{KvError, Kvpair, Value};

mod memory;
pub use memory::MemTable;

mod sleddb;
pub use sleddb::SledDb;
// 对存储的抽象，我们不关心数据存在哪里，但是需要定义如何跟存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，如果已存在返回旧的 value
    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从 HashTable 中删除一个 key，并返回
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = Kvpair;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|item| item.into())
    }
}

#[cfg(test)]
mod test {

    use crate::{Kvpair, Storage};

    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basic_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    pub fn test_basic_interface(store: impl Storage) {
        // let v = store.set("t1", "k1", "v1");
        // assert!(v.unwrap().is_none());

        // let v1 = store.set("t1", "k1", "v2");
        // assert_eq!(v1, Ok(Some("v1".into())));

        // let v = store.get("t1", "k1");
        // assert_eq!(v, Ok(Some("v2".into())));

        // let v = store.contains("t1", "k1");
        // assert_eq!(v, Ok(true));

        // let v = store.del("t1", "k1");
        // assert_eq!(v, Ok(Some("v2".into())));
        // let v = store.contains("t1", "k1");
        // assert_eq!(v, Ok(false));
    }

    pub fn test_get_all(store: impl Storage) {
        store.set("t2", "k1", "v1").unwrap();
        store.set("t2", "k2", "v2").unwrap();

        let mut allv = store.get_all("t2").unwrap();
        allv.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            allv,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    pub fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1", "v1").unwrap();
        store.set("t2", "k2", "v2").unwrap();

        let mut allv: Vec<_> = store.get_iter("t2").unwrap().collect();
        allv.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            allv,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into()),
            ]
        )
    }

    #[test]
    fn memtable_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }
}

#[cfg(test)]
mod test_sleddb {
    use crate::storage::test::*;

    use tempfile::tempdir;

    use super::sleddb::SledDb;

    #[test]
    fn sleddb_basic_interface_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_basic_interface(store)
    }

    #[test]
    fn sleddb_get_all_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_get_all(store)
    }

    #[test]
    fn sleddb_iter_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_get_iter(store)
    }
}
