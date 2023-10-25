use std::ops::{Add, Range, Sub};

use rand::Rng;

mod query;
mod search_struct;

mod leetcode_120;
mod leetcode_128;
mod leetcode_139;
mod leetcode_198;
mod leetcode_2;
mod leetcode_2140;
mod leetcode_219;
mod leetcode_221;
mod leetcode_26;
mod leetcode_27;
mod leetcode_279;
mod leetcode_322;
mod leetcode_36;
mod leetcode_377;
mod leetcode_383;
mod leetcode_416;
mod leetcode_452;
mod leetcode_474;
mod leetcode_48;
mod leetcode_49;
mod leetcode_5;
mod leetcode_516;
mod leetcode_518;
mod leetcode_54;
mod leetcode_56;
mod leetcode_57;
mod leetcode_62;
mod leetcode_63;
mod leetcode_64;
mod leetcode_70;
mod leetcode_73;
mod leetcode_740;
mod leetcode_746;
mod leetcode_80;
mod leetcode_88;
mod leetcode_91;
mod leetcode_931;
mod offer06;
mod solve;
mod sort;

mod MinStack;

mod queue;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct User {
    pub age: i32,
    // pub age_info: String,
}

impl Add for User {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            age: self.age + other.age,
        }
    }
}

impl Sub for User {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            age: self.age - other.age,
        }
    }
}

pub fn create_user_list(n: usize, range: Range<usize>) -> Vec<User> {
    let mut user_list: Vec<User> = Vec::with_capacity(n);
    create_rand_list(n, range, |rand| {
        user_list.push(User {
            age: rand as i32,
            // age_info: format!("my age {}", rand).into(),
        })
    });

    user_list
}

pub fn create_rand_list<F>(n: usize, range: Range<usize>, mut callBack: F)
where
    F: FnMut(usize),
{
    let mut rand: usize = 0;
    let mut rand_arr: Vec<usize> = Vec::with_capacity(n);
    while rand_arr.len() < n {
        rand = rand::thread_rng().gen_range(range.clone());
        let mut flag = false;
        for item in rand_arr.iter() {
            if *item == rand {
                flag = true;
            }
        }
        if !flag {
            rand_arr.push(rand);
            callBack(rand)
        }
    }
}

pub fn Max<T: PartialOrd>(f: T, b: T) -> T {
    return match f > b {
        true => f,
        false => b,
    };
}
