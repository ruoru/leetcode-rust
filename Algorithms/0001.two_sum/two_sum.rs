use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut reverse_nums = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            let num = target - *value;
            if reverse_nums.contains_key(&num) {
                return vec![reverse_nums[&num], index as i32];
            }
            reverse_nums.insert(value, index as i32);
        }
        return vec![];
    }
}
