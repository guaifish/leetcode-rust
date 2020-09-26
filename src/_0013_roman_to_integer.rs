#![allow(dead_code)]

/// Description: https://leetcode.com/problems/roman-to-integer/
/// Best Solution: https://leetcode.com/problems/roman-to-integer/discuss/374718/Rust-pattern-matching-without-extra-allocation
/// My Solution: https://leetcode.com/problems/roman-to-integer/discuss/863979/Elegant-Rust-functional-solution-0ms-2.1MB

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars().rfold(0, |acc, c| {
            acc + match c {
                'I' if acc >= 5 => -1,
                'I' => 1,
                'V' => 5,
                'X' if acc >= 50 => -10,
                'X' => 10,
                'L' => 50,
                'C' if acc >= 500 => -100,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        })
    }
}

struct Solution;

#[test]
fn example1() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
fn example2() {
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
}

#[test]
fn example3() {
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
}

#[test]
fn example4() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
fn example5() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
