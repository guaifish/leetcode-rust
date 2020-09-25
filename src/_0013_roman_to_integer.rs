#![allow(dead_code)]

/// Description: https://leetcode.com/problems/roman-to-integer/
/// [Original]: https://leetcode.com/problems/roman-to-integer/discuss/863979/Elegant-Rust-functional-solution-0ms-2.1MB

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .rev()
            .map(|c| match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            })
            .scan(i32::min_value(), |state, x| {
                let result = if x < *state { -x } else { x };
                *state = x;
                Some(result)
            })
            .sum()
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
