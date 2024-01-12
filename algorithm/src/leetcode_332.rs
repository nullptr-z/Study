use std::mem::swap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn backtracking(
            tickets: &Vec<Vec<String>>,
            used: &mut Vec<bool>,
            result: &mut Vec<String>,
            temp: &mut Vec<String>,
        ) {
            if temp.len() == tickets.len() + 1 {
                if temp < result || result.is_empty() {
                    swap(result, &mut temp.to_vec());
                }
                return;
            }
            for (i, item) in tickets.iter().enumerate() {
                if temp.is_empty() && item[0] != "JFK" {
                    continue;
                }
                if used[i] || !temp.is_empty() && item[0] != *temp.last().unwrap() {
                    continue;
                }
                if !result.is_empty() && temp > result {
                    continue;
                }
                used[i] = true;
                if !temp.is_empty() {
                    temp.pop();
                }
                temp.extend(item.to_vec());
                backtracking(tickets, used, result, temp);
                temp.pop();
                used[i] = false;
            }
        }
        let mut used = vec![false; tickets.len()];
        let mut result = vec![];
        let mut temp = vec![];
        backtracking(&tickets, &mut used, &mut result, &mut temp);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn should_work() {
        // let ret = Solution::find_itinerary(vec![
        //     ["MUC".to_string(), "LHR".to_string()].into(),
        //     ["JFK".to_string(), "MUC".to_string()].into(),
        //     ["SFO".to_string(), "SJC".to_string()].into(),
        //     ["LHR".to_string(), "SFO".to_string()].into(),
        // ]);
        // let ret = Solution::find_itinerary(vec![
        //     ["JFK".to_string(), "SFO".to_string()].into(),
        //     ["JFK".to_string(), "ATL".to_string()].into(),
        //     ["SFO".to_string(), "ATL".to_string()].into(),
        //     ["ATL".to_string(), "JFK".to_string()].into(),
        //     ["ATL".to_string(), "SFO".to_string()].into(),
        // ]);
        // let ret = Solution::find_itinerary(vec![
        //     vec!["EZE".to_string(), "AXA".to_string()],
        //     vec!["TIA".to_string(), "ANU".to_string()],
        //     vec!["ANU".to_string(), "JFK".to_string()],
        //     vec!["JFK".to_string(), "ANU".to_string()],
        //     vec!["ANU".to_string(), "EZE".to_string()],
        //     vec!["TIA".to_string(), "ANU".to_string()],
        //     vec!["AXA".to_string(), "TIA".to_string()],
        //     vec!["TIA".to_string(), "JFK".to_string()],
        //     vec!["ANU".to_string(), "TIA".to_string()],
        //     vec!["JFK".to_string(), "TIA".to_string()],
        // ]);
        let ret = Solution::find_itinerary(vec![
            vec!["AXA".to_string(), "EZE".to_string()],
            vec!["EZE".to_string(), "AUA".to_string()],
            vec!["ADL".to_string(), "JFK".to_string()],
            vec!["ADL".to_string(), "TIA".to_string()],
            vec!["AUA".to_string(), "AXA".to_string()],
            vec!["EZE".to_string(), "TIA".to_string()],
            vec!["EZE".to_string(), "TIA".to_string()],
            vec!["AXA".to_string(), "EZE".to_string()],
            vec!["EZE".to_string(), "ADL".to_string()],
            vec!["ANU".to_string(), "EZE".to_string()],
            vec!["TIA".to_string(), "EZE".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["AUA".to_string(), "JFK".to_string()],
            vec!["JFK".to_string(), "EZE".to_string()],
            vec!["EZE".to_string(), "ANU".to_string()],
            vec!["ADL".to_string(), "AUA".to_string()],
            vec!["ANU".to_string(), "AXA".to_string()],
            vec!["AXA".to_string(), "ADL".to_string()],
            vec!["AUA".to_string(), "JFK".to_string()],
            vec!["EZE".to_string(), "ADL".to_string()],
            vec!["ANU".to_string(), "TIA".to_string()],
            vec!["AUA".to_string(), "JFK".to_string()],
            vec!["TIA".to_string(), "JFK".to_string()],
            vec!["EZE".to_string(), "AUA".to_string()],
            vec!["AXA".to_string(), "EZE".to_string()],
            vec!["AUA".to_string(), "ANU".to_string()],
            vec!["ADL".to_string(), "AXA".to_string()],
            vec!["EZE".to_string(), "ADL".to_string()],
            vec!["AUA".to_string(), "ANU".to_string()],
            vec!["AXA".to_string(), "EZE".to_string()],
            vec!["TIA".to_string(), "AUA".to_string()],
            vec!["AXA".to_string(), "EZE".to_string()],
            vec!["AUA".to_string(), "SYD".to_string()],
            vec!["ADL".to_string(), "JFK".to_string()],
            vec!["EZE".to_string(), "AUA".to_string()],
            vec!["ADL".to_string(), "ANU".to_string()],
            vec!["AUA".to_string(), "TIA".to_string()],
            vec!["ADL".to_string(), "EZE".to_string()],
            vec!["TIA".to_string(), "JFK".to_string()],
            vec!["AXA".to_string(), "ANU".to_string()],
            vec!["JFK".to_string(), "AXA".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["ADL".to_string(), "EZE".to_string()],
            vec!["AXA".to_string(), "TIA".to_string()],
            vec!["JFK".to_string(), "AUA".to_string()],
            vec!["ADL".to_string(), "EZE".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["ADL".to_string(), "AXA".to_string()],
            vec!["TIA".to_string(), "AUA".to_string()],
            vec!["AXA".to_string(), "JFK".to_string()],
            vec!["ADL".to_string(), "AUA".to_string()],
            vec!["TIA".to_string(), "JFK".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["ANU".to_string(), "AXA".to_string()],
            vec!["TIA".to_string(), "AXA".to_string()],
            vec!["EZE".to_string(), "JFK".to_string()],
            vec!["EZE".to_string(), "AXA".to_string()],
            vec!["ADL".to_string(), "TIA".to_string()],
            vec!["JFK".to_string(), "AUA".to_string()],
            vec!["TIA".to_string(), "EZE".to_string()],
            vec!["EZE".to_string(), "ADL".to_string()],
            vec!["JFK".to_string(), "ANU".to_string()],
            vec!["TIA".to_string(), "AUA".to_string()],
            vec!["EZE".to_string(), "ADL".to_string()],
            vec!["ADL".to_string(), "JFK".to_string()],
            vec!["ANU".to_string(), "AXA".to_string()],
            vec!["AUA".to_string(), "AXA".to_string()],
            vec!["ANU".to_string(), "EZE".to_string()],
            vec!["ADL".to_string(), "AXA".to_string()],
            vec!["ANU".to_string(), "AXA".to_string()],
            vec!["TIA".to_string(), "ADL".to_string()],
            vec!["JFK".to_string(), "ADL".to_string()],
            vec!["JFK".to_string(), "TIA".to_string()],
            vec!["AUA".to_string(), "ADL".to_string()],
            vec!["AUA".to_string(), "TIA".to_string()],
            vec!["TIA".to_string(), "JFK".to_string()],
            vec!["EZE".to_string(), "JFK".to_string()],
            vec!["AUA".to_string(), "ADL".to_string()],
            vec!["ADL".to_string(), "AUA".to_string()],
            vec!["EZE".to_string(), "ANU".to_string()],
            vec!["ADL".to_string(), "ANU".to_string()],
            vec!["AUA".to_string(), "AXA".to_string()],
            vec!["AXA".to_string(), "TIA".to_string()],
            vec!["AXA".to_string(), "TIA".to_string()],
            vec!["ADL".to_string(), "AXA".to_string()],
            vec!["EZE".to_string(), "AXA".to_string()],
            vec!["AXA".to_string(), "JFK".to_string()],
            vec!["JFK".to_string(), "AUA".to_string()],
            vec!["ANU".to_string(), "ADL".to_string()],
            vec!["AXA".to_string(), "TIA".to_string()],
            vec!["ANU".to_string(), "AUA".to_string()],
            vec!["JFK".to_string(), "EZE".to_string()],
            vec!["AXA".to_string(), "ADL".to_string()],
            vec!["TIA".to_string(), "EZE".to_string()],
            vec!["JFK".to_string(), "AXA".to_string()],
            vec!["AXA".to_string(), "ADL".to_string()],
            vec!["EZE".to_string(), "AUA".to_string()],
            vec!["AXA".to_string(), "ANU".to_string()],
            vec!["ADL".to_string(), "EZE".to_string()],
            vec!["AUA".to_string(), "EZE".to_string()],
        ]);
        println!("【 ret 】==> {:?}", ret);
    }
}

pub struct Solution;
