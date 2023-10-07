use std::{borrow::BorrowMut, cmp::Ordering, mem::swap, ops::Add};

use crate::Max;

/**
 * 尝试物品1~N种组合,1~M容量情况下能装入物品的最大的价值
 */
pub fn programing_backpack<T>(volume_max: u32, things: Vec<T>) -> Vec<Vec<T>>
where
    // F: Fn(T, u32) -> bool,
    T: Default + Add<Output = T> + PartialOrd + Clone + Into<u32>,
{
    let count = things.len(); // 物品总数量

    let init_volume = vec![T::default(); (volume_max + 1) as usize];
    let mut dp = vec![init_volume; count + 1];

    for cnt in 1..=count {
        for vlu_num in 1..=volume_max {
            let index = cnt - 1;
            let vlu = vlu_num as usize;
            let things_weight = things[index].clone().into();
            // @1:
            dp[cnt][vlu] = match vlu_num >= things_weight {
                true => {
                    let first = dp[index][vlu].clone();
                    // 减去@1 以后才是可以进行对比的位置
                    // 由于@1 跳过了重量,所以需要减去,当前物品重量,确保不会超出剩余容量
                    let price_i = (vlu_num - things_weight) as usize;
                    // 保证剩余容量能够装下当前物品的情况,加上已装的物品
                    let before = dp[index][price_i].clone() + things[index].clone();

                    Max(first, before)
                }
                false => dp[index][vlu].clone(),
            }
        }
    }

    dp
}

#[cfg(test)]
mod tests {
    use crate::solve::dyn_programing_backpack::Thing;

    use super::programing_backpack;

    #[test]
    fn create_dyn_programing_backpack() {
        let things = vec![
            Thing::new(4, 7, 0),
            Thing::new(4, 5, 4),
            Thing::new(1, 2, 1),
            Thing::new(2, 4, 2),
            Thing::new(3, 4, 3),
        ];
        let volume_max = 7u32; // 背包容量

        let result = programing_backpack(volume_max, things);

        assert_eq!(result.len(), result.len());
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Thing {
    pub weight: u32,
    pub price: u32,
    pub code: Vec<usize>,
}

impl PartialOrd for Thing {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let weight_eq = self.weight.partial_cmp(&other.weight);
        let price_eq = self.price.partial_cmp(&other.price);
        if weight_eq == Some(Ordering::Equal) {
            price_eq
        } else {
            weight_eq
        }
    }
}

impl From<Thing> for u32 {
    fn from(thing: Thing) -> Self {
        thing.weight
    }
}

impl Add for Thing {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            weight: self.weight + rhs.weight,
            price: self.price + rhs.price,
            code: [self.code, rhs.code].concat(),
        }
    }
}

impl Thing {
    pub fn new(weight: u32, price: u32, code: usize) -> Self {
        Self {
            weight,
            price,
            code: [[code]].concat(),
        }
    }
}
