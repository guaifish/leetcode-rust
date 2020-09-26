#![allow(dead_code)]

/// Description: https://leetcode.com/problems/longest-common-prefix/
/// Best Solution: https://leetcode.com/problems/longest-common-prefix/discuss/763488/Rust%3A-0ms-solution

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

struct Solution;

#[test]
fn example1() {
    let input = ["flower", "flow", "flight"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::longest_common_prefix(input), "fl");
}

#[test]
fn example2() {
    let input = ["dog", "racecar", "car"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(Solution::longest_common_prefix(input), "");
}
