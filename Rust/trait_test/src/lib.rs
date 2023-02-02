use std::ops::Deref;

#[cfg(test)]
mod test {
    use crate::{As_Goods, Goods};

    fn print(v: impl Into<Goods>) {
        let u: Goods = v.into();
    }

    #[test]
    fn from_Into() {
        let goods = Goods::from(100);
        let to_u32: u32 = goods.into();
        let u32_to_goods: Goods = to_u32.into();
        print(1);

        let as_goods = As_Goods(u32_to_goods.clone());
        assert_eq!(as_goods.idx, u32_to_goods.idx);
        // let aaa: u32 = gggg.into();
    }
}

#[derive(Clone)]
pub struct Goods {
    /** 下标 */
    idx: usize,
    num: usize,
    name: String,
}

impl From<u32> for Goods {
    fn from(a: u32) -> Self {
        Self {
            idx: a as usize,
            num: (a + 1) as usize,
            name: format!("{}", a),
        }
    }
}
// 实现 From 时，自动实现，u32.into( Goods )
// impl Into<Goods> for u32 {
//     fn into(self) -> Goods {
//         u32::from(self);
//     }
// }

impl Into<u32> for Goods {
    fn into(self) -> u32 {
        self.idx as u32
    }
}

struct As_Goods(Goods);

impl Deref for As_Goods {
    type Target = Goods;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
