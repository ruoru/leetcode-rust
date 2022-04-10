pub struct Solution {}

// fn factorial(num: i32) -> i32 {
//     if num == 0 {
//         return 1;
//     }

//     let mut result = 1;
//     for i in 1..(num + 1) {
//         result *= i;
//     }

//     return result;
// }

//fn combination(c: i32, m: i32) -> i32 {
//    return factorial(c) / (factorial(c - m) * factorial(m));
//}

fn combination(c: i32, m: i32) -> i32 {
    let mut cv: u128 = 1;
    let mut mv: u128 = 1;
    for i in (c - m + 1)..(c + 1) {
        cv *= i as u128;
    }

    for i in 1..(m + 1) {
        mv *= i as u128;
    }

    let r = cv / mv;
    return r as i32;
}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut v = vec![];

        let _numrows = num_rows + 1;

        for i in 0..num_rows {
            let mut sv : Vec<i32> = Vec::new();
            for j in 0..(i+1) {
                sv.push(combination(i, j));
            }
            v.push(sv);
        }

        return v;
    }
}