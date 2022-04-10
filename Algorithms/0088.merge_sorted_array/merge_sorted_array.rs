pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut v = vec![];
        let mut i: usize = 0;
        let mut j: usize = 0;
        let _m: usize = if m as usize > nums1.len() {
            nums1.len()
        } else {
            m as usize
        };
        let _n: usize = if n as usize > nums2.len() {
            nums2.len()
        } else {
            n as usize
        };

        while i < _m || j < _n {
            if i == _m && j < _n {
                v.push(nums2[j]);
                j += 1;
                continue;
            } else if i < _m && j == _n {
                v.push(nums1[i]);
                i += 1;
                continue;
            }

            let v1 = nums1[i];
            let v2 = nums2[j];

            if v1 < v2 {
                i += 1;
                v.push(v1);
            } else if v1 > v2 {
                j += 1;
                v.push(v2);
            } else {
                i += 1;
                j += 1;
                v.push(v1);
                v.push(v2);
            }
        }

        *nums1 = v;
    }
}
