use std::collections::HashMap;

pub fn two_sum (nums: Vec<i32>, target: i32) -> [i32; 2] {
    let mut nums_indexs = HashMap::new();
    let mut key = 0;
    for value in nums.iter() {
        let nums_index = target - value;
        if nums_indexs.contains_key(&nums_index) {
            return [nums_indexs[&nums_index], key]
        }
        nums_indexs.insert(value, key);
        key += 1;
    }
    return [0, 0];
}

fn main() {
    let _nums = vec![2, 7, 9, 13];
    let _target = 9;
    let results = two_sum(_nums, _target);
    println!("{:?}", results);
}