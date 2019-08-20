mod two_sum;
use two_sum::Solution;

fn main() {
    let results = Solution::two_sum(vec![2, 7, 9, 13], 9);
    println!("{:?}", results);
}
