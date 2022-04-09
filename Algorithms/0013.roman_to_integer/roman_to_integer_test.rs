mod roman_to_integer;
use roman_to_integer::Solution;

fn main() {
    let results = Solution::roman_to_int("MCMXCIV".to_owned());
    println!("{:?}", results);
}
