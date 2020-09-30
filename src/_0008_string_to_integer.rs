use crate::Solution;

/// Description: https://leetcode.com/problems/string-to-integer-atoi/
///
/// My Solution: https://leetcode.com/problems/string-to-integer-atoi/discuss/871326/Rust-solution-using-recursion-easy-understand-0ms-1.9MB

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        parse(extract(str.as_bytes(), 0, 0))
    }
}

fn extract(s: &[u8], offset: usize, sign: i8) -> (&[u8], bool) {
    match s.is_empty() {
        true => (b"0", false),
        _ => match s[offset] {
            b' ' if sign == 0 && offset == 0 => extract(&s[1..], 0, 0),
            b'-' if sign == 0 && offset == 0 => extract(&s[1..], 0, -1),
            b'+' if sign == 0 && offset == 0 => extract(&s[1..], 0, 1),
            b'0'..=b'9' if offset < s.len() - 1 => extract(s, offset + 1, sign),
            b'0'..=b'9' if sign == -1 => (s, true),
            b'0'..=b'9' => (s, false),
            _ if offset == 0 => (b"0", false),
            _ if sign == -1 => (&s[0..offset], true),
            _ => (&s[0..offset], false),
        },
    }
}

fn parse((s, neg): (&[u8], bool)) -> i32 {
    let sign = if neg { -1 } else { 1 };
    let mut total: i64 = 0;
    let min = -2_i64.pow(31);
    let max = 2_i64.pow(31) - 1;
    for &i in s {
        let n = match i {
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            b'3' => 3,
            b'4' => 4,
            b'5' => 5,
            b'6' => 6,
            b'7' => 7,
            b'8' => 8,
            b'9' => 9,
            _ => unimplemented!(),
        };
        total = total * 10 + sign * n;
        if total < min {
            return min as i32;
        } else if total > max {
            return max as i32;
        }
    }
    total as i32
}
