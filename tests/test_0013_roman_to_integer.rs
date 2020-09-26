#[macro_use]
mod macros;

leetcode_test! {
    roman_to_int:
        example1("III".to_string()) -> 3;
        example2("IV".to_string()) -> 4;
        example3("IX".to_string()) -> 9;
        example4("LVIII".to_string()) -> 58;
        example5("MCMXCIV".to_string()) -> 1994;
}
