mod merge_sorted_array;
use merge_sorted_array::Solution;

fn main() {
    let mut vec1 = vec![9, 13, 17, 19];
    let mut vec2 = vec![2, 7, 9, 14, 15, 24, 28, 38];

    Solution::merge(&mut vec1, 4,&mut vec2, 5);
    println!("{:?}", vec1);
}
