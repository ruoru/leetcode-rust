mod pascals_triangle;
use pascals_triangle::Solution;

fn main() {
    let results = Solution::generate(30);
    println!("{:?}", results);
}
