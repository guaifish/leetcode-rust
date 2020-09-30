#[macro_use]
mod macros;

leetcode_test! {
    my_atoi:
        example1("42".to_string()) -> 42;
        example2("   -42".to_string()) -> -42;
        example3("4193 with words".to_string()) -> 4193;
        example4("words and 987".to_string()) -> 0;
        example5("-91283472332".to_string()) -> -2147483648;
        example6("20000000000000000000".to_string()) -> 2147483647;
        example7("-   234".to_string()) -> 0;
        example8("+1".to_string()) -> 1;
}
