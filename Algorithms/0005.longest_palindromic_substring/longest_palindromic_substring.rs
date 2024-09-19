impl Solution {
    pub fn longest_palindrome(s: String) -> String {

        let n = s.len();
        if n < 2 {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut max_len = 1;

        let expand_around_center = |left: i32, right: i32| -> (usize, usize) {
            let mut l = left;
            let mut r = right;

            while l >= 0 && r < n as i32 && s_chars[l as usize] == s_chars[r as usize] {
                l -= 1;
                r += 1;
            }

            ((l + 1) as usize, (r - l - 1) as usize)
        };

        for i in 0..n {
            let (start1, len1) = expand_around_center(i as i32, i as i32);
            let (start2, len2) = expand_around_center(i as i32, (i + 1) as i32);

            if len1 > max_len {
                start = start1;
                max_len = len1;
            }
        }
        
        s[start..start + max_len].to_string()
    }
}