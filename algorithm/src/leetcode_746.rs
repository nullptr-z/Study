//

pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut pre_min = cost[0];
        let mut current_min = cost[1];
        for i in 2..cost.len() {
            // pre_min = [i-2]轮那一次的值, current_min = [i-1]轮那一次的值
            // 最新一轮中，累计后的最小值
            let sum_min = (pre_min + cost[i]).min(current_min + cost[i]);
            pre_min = current_min;
            current_min = sum_min;
        }

        pre_min.min(current_min)
    }

    pub fn min_cost_climbing_stairs_3(mut costs: Vec<i32>) -> i32 {
        let mut money = 0;
        let mut idx = 0;
        let mut temp: i32 = 0;
        if costs.len() <= 3 {
            costs.push(0);
            if costs.len() <= 3 {
                costs.push(0);
            }
        }
        while idx < (costs.len() - 3) {
            let mut cost = costs[idx..(idx + 4)].to_vec();
            println!("【 cost 】==> {:?}", cost);
            let i = 0;
            let a01 = cost[i] + cost[i + 1];
            let a02 = cost[i] + cost[i + 2];
            let a03 = cost[i] + cost[i + 3];
            let a12 = cost[i + 1] + cost[i + 2];
            let a13 = cost[i + 1] + cost[i + 3];
            // let a23 = cost[i + 2] + cost[i + 3];

            let mut min = a02.min(a13);
            // min = min.min(a12);
            // min = min.min(a01);
            // min = min.min(a03);
            // min = min.min(a23);

            money += min;

            idx += 4;
        }
        println!("【 money 】==> {:?}", money);
        money
    }

    pub fn min_cost_climbing_stairs_2(mut cost: Vec<i32>) -> i32 {
        let mut money = 0;
        let mut i = 0;
        let mut temp: i32 = 0;
        cost.push(0);
        while i < (cost.len() - 1) {
            if cost[i + 1] < cost[i + 2] {
                money += cost[i];
                if i + 2 == cost.len() {
                    money -= temp
                }

                temp = cost[i];
            } else {
                money += cost[i + 1];
                temp = cost[i + 1];
            }
            println!("【 temp 】==> {:?}", temp);
            i += 1;
        }
        println!("【 money 】==> {:?}", money);
        money
    }

    pub fn min_cost_climbing_stairs_1(cost: Vec<i32>) -> i32 {
        let mut money = 0;
        let mut i = 0;
        let mut select = vec![];
        while i < (cost.len()) {
            if cost.len() - i == 2 {
                if cost[i] < cost[i + 1] {
                    select.push(cost[i]);
                    money += cost[i];
                } else {
                    select.push(cost[i + 1]);
                    money += cost[i + 1];
                }
                i += 2;
                break;
            } else if cost.len() - i == 1 {
                println!("【 i 】==> {:?}", i);
                select.push(cost[i]);
                money += cost[i];
                i += 1;
                break;
            }

            // let a1 = cost[i] + cost[i + 1];
            // let a2 = cost[i] + cost[i + 2];
            // let a3 = cost[i + 1] + cost[i + 2];
            // if a1 < a2 && a1 < a3 {
            //     select.push(cost[i]);
            //     select.push(cost[i + 1]);
            // } else if a2 < a3 {
            //     select.push(cost[i]);
            //     select.push(cost[i + 2]);
            // } else {
            //     select.push(cost[i + 1]);
            //     select.push(cost[i + 2]);
            // }

            if cost[i + 1] > cost[i + 2] && (cost[i] < cost[i + 1] || cost[i] < cost[i + 2]) {
                select.push(cost[i]);
                money += cost[i];
            }
            if cost[i + 1] < cost[i + 2] {
                select.push(cost[i + 1]);
                money += cost[i + 1];
            } else {
                select.push(cost[i + 2]);
                money += cost[i + 2];
            }

            i += 3;

            // money += select[select.len() - 1];
            // money += select[select.len() - 2];
        }

        println!("【 select 】==> {:?},i={},money={}", select, i, money);

        money
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        // Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
        // Solution::min_cost_climbing_stairs(vec![1, 100]);
    }

    #[test]
    fn should_work2() {
        // [1,100,1,1,1,100,1,1,100,1]
        // Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
        // Solution::min_cost_climbing_stairs(vec![0, 0, 1, 1]);
    }
}

// 1 1 1
// 1 1 0
// 1 0 1
// 1 0 0
// 0 1 1
// 0 1 0
// 0 0 1
