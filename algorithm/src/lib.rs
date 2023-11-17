use std::ops::{Add, Range, Sub};

use rand::Rng;

mod query;
mod search_struct;

mod array_utils;
mod list_utils;
mod tree_utils;

mod leetcode_101;
mod leetcode_102;
mod leetcode_103;
mod leetcode_104;
mod leetcode_105;
mod leetcode_106;
mod leetcode_107;
mod leetcode_108;
mod leetcode_11;
mod leetcode_112;
mod leetcode_114;
mod leetcode_120;
mod leetcode_121;
mod leetcode_124;
mod leetcode_125;
mod leetcode_128;
mod leetcode_129;
mod leetcode_134;
mod leetcode_136;
mod leetcode_139;
mod leetcode_14;
mod leetcode_144;
mod leetcode_145;
mod leetcode_146;
mod leetcode_148;
mod leetcode_15;
mod leetcode_151;
mod leetcode_153;
mod leetcode_162;
mod leetcode_167;
mod leetcode_169;
mod leetcode_17;
mod leetcode_172;
mod leetcode_173;
mod leetcode_173_stack;
mod leetcode_189;
mod leetcode_19;
mod leetcode_198;
mod leetcode_199;
mod leetcode_2;
mod leetcode_200;
mod leetcode_21;
mod leetcode_2140;
mod leetcode_215;
mod leetcode_219;
mod leetcode_221;
mod leetcode_222;
mod leetcode_226;
mod leetcode_229;
mod leetcode_230;
mod leetcode_236;
mod leetcode_26;
mod leetcode_27;
mod leetcode_279;
mod leetcode_28;
mod leetcode_300;
mod leetcode_322;
mod leetcode_33;
mod leetcode_34;
mod leetcode_35;
mod leetcode_36;
mod leetcode_377;
mod leetcode_383;
mod leetcode_392;
mod leetcode_416;
mod leetcode_452;
mod leetcode_474;
mod leetcode_48;
mod leetcode_49;
mod leetcode_5;
mod leetcode_50;
mod leetcode_516;
mod leetcode_518;
mod leetcode_530;
mod leetcode_54;
mod leetcode_56;
mod leetcode_57;
mod leetcode_58;
mod leetcode_61;
mod leetcode_62;
mod leetcode_63;
mod leetcode_637;
mod leetcode_64;
mod leetcode_66;
mod leetcode_70;
mod leetcode_73;
mod leetcode_74;
mod leetcode_740;
mod leetcode_746;
mod leetcode_80;
mod leetcode_82;
mod leetcode_88;
mod leetcode_9;
mod leetcode_91;
mod leetcode_92;
mod leetcode_931;
mod leetcode_94;
mod leetcode_98;
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
