#![allow(unused)]

macro_rules! vec_string {
    ($elem:expr; $n:expr) => (
        vec![$elem.to_string(); $n]
    );
    ($($x:expr),+ $(,)?) => (
        vec![$($x.to_string()),+]
    );
}

macro_rules! test {
    ($function:ident, $test_name:ident, $input:expr, $output:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!(Solution::$function($input), $output);
        }
    };
}

macro_rules! leetcode_test {
    ($function:ident: $($test_name:ident($input:expr) -> $output:expr);+ $(;)?) => {
        use leetcode_rust::Solution;
        $(test!{$function, $test_name, $input, $output})+
    };
}
