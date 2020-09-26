#[macro_use]
mod macros;

leetcode_test! {
    longest_common_prefix:
        example1(vec_string!["flower", "flow", "flight"]) -> "fl".to_string();
        example2(vec_string!["dog", "racecar", "car"]) -> "".to_string();
}
