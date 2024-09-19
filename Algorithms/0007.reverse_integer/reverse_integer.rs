impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut rev = 0;

        while x != 0 {
            let pop = x % 10;
            x /= 10;
            rev = rev * 10 + pop;
        }
        rev
    }
}