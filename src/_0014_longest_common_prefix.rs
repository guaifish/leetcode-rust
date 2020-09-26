/// Description: https://leetcode.com/problems/longest-common-prefix/
/// Best Solution: https://leetcode.com/problems/longest-common-prefix/discuss/763488/Rust%3A-0ms-solution
use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            }),
        }
    }
}
