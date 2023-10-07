use std::{collections::VecDeque, default, f32::consts::E};

use crate::{
    search_struct::{PriorityQueue, PRIORITY},
    sort::MergeSort,
    Max,
};

const N: u32 = 2 << n + 1; // 足够大的数组容量
const n: usize = 4; // 物品数量
const m: i32 = 12; // 背包容量;上界
const v: [i32; n + 2] = [0, 1, 2, 3, 4, 0]; // 物品i重量
const w: [i32; n + 2] = [0, 2, 4, 4, 5, 0]; // 物品i价值
const s: [i32; n + 2] = [0, 2, 6, 10, 15, 0]; // 1~i物品价值之和

fn create_branch_method_backpack() -> i32 {
    let mut res: i32 = -1; // 下界

    let goods_init = Goods::default();
    let mut goods = vec![goods_init; N.try_into().unwrap()];
    goods[1].idx = 1;

    let mut bbm: VecDeque<Goods> = VecDeque::with_capacity(N.try_into().unwrap());
    bbm.push_back(goods[1]);
    while bbm.len() > 0 {
        let t = bbm.pop_front().unwrap();
        let goods_top = goods[t.idx];

        let idx = t.idx << 1;
        let log2 = f32::log2(idx as f32) as usize;

        goods[idx] = Goods {
            idx: idx,
            c: goods_top.c,
            r: s[n] - s[log2],
            tv: goods_top.tv,
            rate: -1.0,
        };

        goods[idx + 1] = Goods {
            idx: idx + 1,
            c: goods_top.c + w[log2],
            r: s[n] - s[log2],
            tv: goods_top.tv + v[log2],
            rate: -1.0,
        };

        if f32::log2(goods_top.idx as f32) as usize == n {
            res = Max(res, goods_top.c.try_into().unwrap());
            continue;
        }

        let mut compar_push = |goods_item: Goods| {
            let c_r_sum = (goods_item.c + goods_item.r) as i32;
            if goods_item.tv <= m && c_r_sum > res {
                bbm.push_back(goods_item);
            }
        };
        compar_push(goods[idx]);
        compar_push(goods[idx + 1]);
    }

    res
}

fn create_branch_method_limit_first_backpack(goods_arr: Vec<Goods>, Weight: i32) -> i32 {
    let mut bbm = VecDeque::with_capacity(goods_arr.len());
    let goods_default = Goods::default();

    let mut start_index: usize = 0;
    let goods_first = goods_arr
        .iter()
        .find(|item| {
            if item.tv <= Weight {
                start_index += 1;
                return true;
            }
            false
        })
        .unwrap();
    let r = Weight * goods_first.c / goods_first.tv;
    let up_init = Goods {
        idx: 0,
        c: 0,
        r,
        tv: 0,
        rate: -1.0,
    };
    bbm.push_back(up_init);

    let down = goods_first.c;

    let get_ub = |top: Goods, goods: Goods, goods_next: Goods| -> Goods {
        let m2 = (Weight - goods.tv - top.tv) as i32;
        let m3 = match goods_next.c > 0 {
            true => (goods_next.c / goods_next.tv) as i32,
            false => 0,
        };
        let ub = goods.c + top.c + m2 * m3;

        Goods {
            idx: 0,
            c: top.c + goods.c,
            r: ub,
            tv: top.tv + goods.tv,
            rate: -1.0,
        }
    };

    for i in start_index..goods_arr.len() {
        let top = bbm.pop_front().unwrap();
        let goods_next = match i != goods_arr.len() - 1 {
            true => goods_arr[i + 1],
            false => goods_default,
        };
        let select_goods = get_ub(top, goods_arr[i], goods_next);
        let not_select_goods = get_ub(top, goods_default, goods_next);

        let mut is_optional = |option: Goods| {
            if option.tv <= Weight && option.r >= down && option.r <= top.r {
                bbm.push_back(option);
                return true;
            }
            return false;
        };

        // 因为已经按最大性价比倒序排过了, 所以用 >= 而不是 >
        if select_goods.r >= not_select_goods.r {
            let is_push = is_optional(select_goods);
            if !is_push {
                is_optional(not_select_goods);
            }
        } else {
            is_optional(not_select_goods);
        }
    }
    bbm.pop_front().unwrap().r
}

fn create_branch_method_limit_first_priority_backpack(goods_arr: Vec<Goods>, Weight: i32) -> i32 {
    let len = goods_arr.len();
    let _wite = 2 << len;
    // 汇总用
    let mut goods_total = Goods::new(0, 0, 0, 0);

    let mut pq: PriorityQueue<Goods> =
        PriorityQueue::new(Vec::with_capacity(2 << len), PRIORITY::MaxHeap);

    // 第i个物品的上界
    let get_up_limit = |index: usize, goods_total: &Goods| -> i32 {
        let mut remaining_weight = Weight - goods_total.tv;
        let mut up_bound = goods_total.c;

        for i in index..len {
            let item = goods_arr[i];
            if item.tv <= remaining_weight {
                remaining_weight -= item.tv;
                up_bound += item.c;
            } else {
                up_bound += (item.rate * remaining_weight as f32) as i32;
                break;
            }
        }

        up_bound
    };

    let mut local_best = 0; //当前局部最优值

    let mut i = 0;
    while i < len {
        let that_tv = goods_total.tv + goods_arr[i].tv;
        if that_tv <= Weight {
            let that_c = goods_total.c + goods_arr[i].c;
            if that_c > local_best {
                local_best = that_c;
            }

            pq.push(Goods::new(i + 1, that_c, goods_total.r, that_tv));
        }

        goods_total.r = get_up_limit(i + 1, &goods_total);

        if goods_total.r >= local_best {
            pq.push(Goods::new(
                i + 1,
                goods_total.c,
                goods_total.r,
                goods_total.tv,
            ));
        }

        let top = pq.pop().unwrap();

        goods_total.c = top.c;
        goods_total.tv = top.tv;
        i = top.idx;
    }

    let a = 1;
    goods_total.c
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        solve::{
            branch_bind_method_backpack::{
                create_branch_method_backpack, create_branch_method_limit_first_backpack,
                create_branch_method_limit_first_priority_backpack, Goods,
            },
            dyn_programing_backpack::Thing,
        },
        sort::{MergeSort, QuickSort},
    };

    #[test]
    fn create_dyn_programing_backpack() {
        let now = Instant::now();
        let result = create_branch_method_backpack();
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();
        assert_eq!(result, 15);

        /*-----------初始化 start------------- */
        let Weight = 30;
        let vv = [3, 7, 1, 13, 11, 1, 3, 4, 15, 11, 12, 13, 9, 6, 7, 10];
        let ww = [
            18, 42, 12, 100, 40, 20, 30, 11, 9, 10, 2, 30, 10, 33, 22, 11,
        ];
        let len = vv.len();
        let mut goods_arr = Vec::with_capacity(len);
        for i in 0..len {
            goods_arr.push(Goods::new(i, ww[i], -1, vv[i]));
        }
        QuickSort::new(&mut goods_arr, |f, b| f.rate > b.rate).sort();
        /*-----------初始化 end------------- */

        let now = Instant::now();
        let result = create_branch_method_limit_first_backpack(goods_arr.clone(), Weight);
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();
        assert_eq!(result, 202);

        let now = Instant::now();
        let result = create_branch_method_limit_first_priority_backpack(goods_arr, Weight);
        let micros = now.elapsed().as_micros();
        let millis = now.elapsed().as_millis();
        assert_eq!(result, 204);
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Goods {
    /** 下标 */
    idx: usize,
    /** 价值 */
    c: i32,
    /** 界/剩余总价值 */
    r: i32,
    /** 体积 */
    tv: i32,
    /** 平均 */
    rate: f32,
}

impl PartialOrd for Goods {
    fn lt(&self, other: &Self) -> bool {
        (self.c / self.tv) > (other.c / other.tv)
    }

    fn gt(&self, other: &Self) -> bool {
        self.r > other.r
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.idx.partial_cmp(&other.idx) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.c.partial_cmp(&other.c) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.r.partial_cmp(&other.r) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.tv.partial_cmp(&other.tv)
    }
}

impl Goods {
    pub fn new(idx: usize, c: i32, r: i32, tv: i32) -> Self {
        Self {
            idx,
            c,
            r,
            tv,
            rate: c as f32 / tv as f32,
        }
    }
}
