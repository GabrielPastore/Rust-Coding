fn compare_strings (will_check: &str) -> bool {
    assert_eq!(will_check, reversed_str);
}


#[cfg(test)]
mod check_palindrome{
    #[test]
    fn is_palindrome() {
        let my_string: &str = "onibus";
        compare_strings(my_string);
    }
}
