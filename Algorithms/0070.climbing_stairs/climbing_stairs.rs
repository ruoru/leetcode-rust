use std::collections::HashMap;
pub struct Solution {}

fn loop_stairs(n: i32, cacheways: &mut HashMap<i32, i32>) -> i32 {
    match cacheways.get(&n) {
        Some(v) => return *v,
        None => {}
    }

    let mut n1: i32 = 0;
    match cacheways.get(&n1) {
        Some(v) => n1 = *v,
        None => {
            n1 = loop_stairs(n - 1, cacheways);
            cacheways.insert(n - 1, n1);
        }
    }

    let mut n2: i32 = 0;
    match cacheways.get(&n2) {
        Some(v) => n2 = *v,
        None => {
            n2 = loop_stairs(n - 2, cacheways);
            cacheways.insert(n - 2, n2);
        }
    }

    return n1 + n2;
}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cacheways: HashMap<i32, i32> = HashMap::from([(1, 1), (2, 2)]);

        return loop_stairs(n, &mut cacheways);
    }
}
