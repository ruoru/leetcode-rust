struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut start = 0;
        let mut max_len = 1;

        // 动态规划数组，dp[i][j] 表示 s[i..=j] 是否为回文串
        let mut dp = vec![vec![false; n]; n];

        // 所有长度为 1 的子串都是回文串
        for i in 0..n {
            dp[i][i] = true;
        }

        // 检查长度为 2 及以上的子串
        for len in 2..=n {
            for i in 0..n - len + 1 {
                let j = i + len - 1;
                if len == 2 {
                    dp[i][j] = chars[i] == chars[j];
                } else {
                    dp[i][j] = dp[i + 1][j - 1] && chars[i] == chars[j];
                }

                if dp[i][j] && len > max_len {
                    start = i;
                    max_len = len;
                }
            }
        }

        chars[start..start + max_len].iter().collect()
    }
}

fn main() {
    let result = Solution::longest_palindrome("babad".to_string());
    let result2 = Solution::longest_palindrome("abbcccddcccbba".to_string());
    println!("{}, {}", result, result2);
}
