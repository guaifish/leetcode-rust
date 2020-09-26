#![allow(dead_code)]

/// Description: https://leetcode.com/problems/palindrome-number/
/// Best Solution: https://leetcode.com/problems/palindrome-number/discuss/333683/Rust-0ms-4ms
/// Runtime: 0 ms   Memory: 2.8 MB

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let digit_count = 1 + (x as f32).log10() as u32;
        let half = (digit_count / 2) as usize;
        let digits = (0..digit_count).map(|exp| x / 10_i32.pow(exp) % 10);
        digits
            .clone()
            .take(half)
            .zip(digits.rev().take(half))
            .all(|(lhs, rhs)| lhs == rhs)
    }
}

struct Solution;

#[test]
fn example1() {
    assert_eq!(Solution::is_palindrome(121), true);
}

#[test]
fn example2() {
    assert_eq!(Solution::is_palindrome(-121), false);
}

#[test]
fn example3() {
    assert_eq!(Solution::is_palindrome(10), false);
}
