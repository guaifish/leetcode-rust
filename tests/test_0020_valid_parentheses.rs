#[macro_use]
mod macros;

leetcode_test! {
    is_valid:
        example1("()".to_string()) -> true;
        example2("()[]{}".to_string()) -> true;
        example3("(]".to_string()) -> false;
        example4("([)]".to_string()) -> false;
        example5("{[]}".to_string()) -> true;
}
