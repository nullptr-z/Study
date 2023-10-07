const MaxHashCount: usize = 1013;
struct HashTable<T> {
    table: Vec<Option<T>>,
}

impl<T: Default> Default for HashTable<T> {
    fn default() -> Self {
        let mut selfs = Self {
            table: Vec::with_capacity(MaxHashCount),
        };
        for i in 0..MaxHashCount {
            selfs.table.push(Some(T::default()));
        }
        selfs
    }
}

impl<T: Clone + Default + PartialEq> HashTable<T> {
    fn get_hash_code(&self, key: &str) -> usize {
        let mut code: usize = 5381;
        for k in key.as_bytes() {
            code = (code << 5) + *k as usize;
        }
        code % MaxHashCount
    }

    fn hash_table(&self) {}

    pub fn get(&self, key: &str) -> Option<T> {
        let hashCode = self.get_hash_code(key);
        let item = self.table[hashCode].as_ref().unwrap();
        Some(item.clone())
    }

    pub fn set(&mut self, key: &str, value: &T) -> bool {
        let hashCode = self.get_hash_code(key);
        let mut result = self.table[hashCode] == Some(T::default());
        self.table[hashCode] = if result {
            result = true;
            Some(value.clone())
        } else {
            todo!()
        };
        result
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::search_struct::hash::{HashTable, User};

    #[test]
    fn creat_Hash_table() {
        let mut hash = HashTable::default();
        let user: User = User {
            name: "zhouzheng".into(),
            age: 26,
        };
        hash.set("mrzhou".into(), &user);
        let getHashUser = hash.get("mrzhou".into());
        assert_eq!(user, getHashUser.unwrap());
    }
}
