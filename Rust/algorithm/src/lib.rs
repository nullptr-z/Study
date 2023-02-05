use std::{
    fmt::format,
    ops::{Add, Range, Sub},
};

use rand::Rng;

mod query;
mod search_struct;

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
