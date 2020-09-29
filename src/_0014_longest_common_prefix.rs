/// Description: https://leetcode.com/problems/longest-common-prefix/
/// Best Solution: https://leetcode.com/problems/longest-common-prefix/discuss/763488/Rust%3A-0ms-solution
use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        if strs.len() == 1 {
            return strs[0].clone();
        }

        let min_len = strs.iter().map(|x| x.len()).min().unwrap();
        if min_len == 0 {
            return String::new();
        }

        let mut i = 0;
        let s0 = strs[0].as_bytes();
        'o: while i < min_len {
            for s in strs.iter().map(|s| s.as_bytes()).skip(1) {
                if s[i] != s0[i] {
                    break 'o;
                }
            }
            i += 1;
        }

        String::from_utf8_lossy(&s0[..i]).to_string()
    }
}
