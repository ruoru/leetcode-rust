use std::cmp;
use std::mem;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut vec1 = nums1;
        let mut vec2 = nums2;

        if vec1.len() > vec2.len() {
            mem::swap(&mut vec1, &mut vec2);
        }

        let len1 = vec1.len();
        let len2 = vec2.len();

        let half_len = (len1 + len2 + 1) / 2;
        let mut index_min = 0;
        let mut index_max = len1;

        while index_min <= index_max {
            let i = (index_min + index_max) / 2;
            let j = half_len - i;

            if i < len1 && vec1[i] < vec2[j - 1] {
                index_min = i + 1;
            } else if i > 0 && vec1[i-1] > vec2[j] {
                index_max = i - 1;
            } else {
                let left_max;
                let right_min;
                if i == 0 {
                    left_max = vec2[j - 1];
                } else if j == 0 {
                    left_max = vec1[i-1];
                } else {
                    left_max = cmp::max(vec1[i-1], vec2[j-1]);
                }

                if (len1 + len2) % 2 == 1 {
                    return left_max as f64;
                }

                if i == len1 {
                    right_min = vec2[j];
                } else if j == len2 {
                    right_min = vec1[i];
                } else {
                    right_min = cmp::min(vec1[i], vec2[j]);
                }

                return (left_max + right_min) as f64 / 2.0
            }
        }

        return 0.0;
    }
}


fn main() {
    let vec1 = vec![9, 13, 17, 19];
	let vec2 = vec![2, 7, 9, 14, 15, 24, 28, 38];

    let results = median_of_two_sorted_arrays(vec1, vec2);
    println!("{}", results);
}