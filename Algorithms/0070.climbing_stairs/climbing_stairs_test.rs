mod climbing_stairs;
use climbing_stairs::Solution;

fn main() {
    let results = Solution::climb_stairs(4);
    println!("{:?}", results);
}
