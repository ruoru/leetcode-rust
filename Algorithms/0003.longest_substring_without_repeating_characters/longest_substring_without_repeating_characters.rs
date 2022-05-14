use std::cmp::max;
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index = 0;
        let mut prev_index = -1;
        let mut result = 0;
        let mut sub_strings: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            match sub_strings.get(&c) {
                None => {}
                Some(p_ind) => {
                    if prev_index < *p_ind {
                        prev_index = *p_ind;
                    }
                }
            }

            result = max(result, index - prev_index);
            sub_strings.insert(c, index);
            index += 1;
        }

        return result as i32;
    }
}
