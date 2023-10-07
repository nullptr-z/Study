use std::{fmt::Debug, time::Instant};

use anyhow::{anyhow, Ok, Result};
use rand::Rng;

pub fn get_gen_range_array<T: Clone, F>(mut MAX: usize, func: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    MAX = match MAX < 1 {
        true => 100,
        false => MAX,
    };
    let mut mergeArray: Vec<T> = Vec::with_capacity(MAX);
    for i in 0..MAX {
        mergeArray.push(func(i));
    }

    let mut randIndex: usize = 0;
    let mut randIndex1: usize = 0;
    for _ in 0..(MAX * 2) {
        randIndex = rand::thread_rng().gen_range(0..MAX);
        randIndex1 = rand::thread_rng().gen_range(0..MAX);
        if randIndex != randIndex1 {
            let temp = mergeArray[randIndex].clone();
            mergeArray[randIndex] = mergeArray[randIndex1].clone();
            mergeArray[randIndex1] = temp;
        }
    }
    mergeArray
}

pub struct MergeSort<T> {
    MergeArray: Vec<T>,
}

impl<T: PartialOrd + Debug + Clone> MergeSort<T> {
    pub fn new(arr: Vec<T>) -> Self {
        Self { MergeArray: arr }
    }

    fn divide<'a>(&self, f: Vec<&'a T>, b: Vec<&'a T>) -> Vec<&'a T>
    where
        T: 'a,
    {
        let arrLen = f.len();
        let arrLen_b = b.len();

        let (fArr, bArr) = match arrLen > 1 || arrLen_b > 1 {
            true => {
                if arrLen == 1 && arrLen_b > 1 {
                    (
                        f,
                        self.divide(b[0..arrLen_b >> 1].to_vec(), b[arrLen_b >> 1..].to_vec()),
                    )
                } else {
                    // let pRange = 0..arrLen >> 1;
                    // let nRange = arrLen >> 1..arrLen;
                    (
                        self.divide(f[0..arrLen >> 1].to_vec(), f[arrLen >> 1..].to_vec()),
                        self.divide(b[0..arrLen >> 1].to_vec(), b[arrLen >> 1..].to_vec()),
                    )
                }
            }
            false => (f, b),
        };

        let mut temp = Vec::with_capacity(fArr.len() + bArr.len());
        let mut i = 0;
        let mut j = 0;
        loop {
            let fq = &*fArr[i];
            let bq = &*bArr[j];
            if fq < bq {
                temp.push(fArr[i]);
                i += 1;
            } else {
                temp.push(bArr[j]);
                j += 1
            }
            if i == (fArr.len()) {
                // for index in j..bArr.len() {
                //     temp.push(bArr[index])
                // }
                for item in &bArr[j..] {
                    temp.push(item);
                }
                break;
            } else if j == (bArr.len()) {
                // for index in i..fArr.len() {
                //     temp.push(fArr[index])
                // }
                for item in &fArr[i..] {
                    temp.push(item);
                }
                break;
            }
        }

        temp
    }

    pub fn merge_sort(&self) -> Result<Vec<T>> {
        let arrLen = self.MergeArray.len();
        if arrLen < 2usize {
            return Err(anyhow!("数组长度小于不能小于2个数"));
        }

        // let f = 0..(arrLen >> 1);
        // let b = (arrLen >> 1)..arrLen;
        let now = Instant::now();
        let mut addrArray = Vec::with_capacity(arrLen);
        for item in self.MergeArray.iter() {
            addrArray.push(item)
        }
        let consume_time_millis = now.elapsed().as_millis();

        let now = Instant::now();
        let reuslt = self.divide(
            addrArray[0..(arrLen >> 1)].to_vec(),
            addrArray[(arrLen >> 1)..].to_vec(),
        );
        let consume_time_millis = now.elapsed().as_millis();

        let now = Instant::now();
        let mut temp = Vec::with_capacity(arrLen);
        for item in reuslt {
            temp.push(item.clone())
        }
        let consume_time_millis = now.elapsed().as_millis();

        Ok(temp)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Message {
    pub num: usize,
    str: String,
}

impl Message {
    pub fn new(num: usize, str: &str) -> Self {
        Self {
            num,
            str: str.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::sort::merge::MergeSort;

    use super::{get_gen_range_array, Message};

    #[test]
    fn crate_merge_sort() {
        let array = get_gen_range_array(999, |num| {
            Message::new(num, format!("wawa-{}", num).as_str())
            // num
        });

        let merge = MergeSort::new(array);

        let now = Instant::now();
        let temp = merge.merge_sort();
        let consume_time_micros = now.elapsed().as_micros();
        let consume_time_millis = now.elapsed().as_millis();

        for (index, item) in temp.unwrap().iter().enumerate() {
            assert_eq!(item.num, index);
            // assert_eq!(*item, index);
        }
    }
}
