/// Description: https://leetcode.com/problems/valid-parentheses/
/// Best Solution: https://leetcode.com/problems/valid-parentheses/discuss/500491/Rust-0ms
use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(i) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
    }
}
