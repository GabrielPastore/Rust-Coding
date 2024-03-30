use regex::Regex;


// Palíndromo
fn compare_strings (input_string: &str) -> bool {
    let cleaner = Regex::new(r"[^A-Za-z]").unwrap();
    let clean_input_string = cleaner.replace_all(input_string, "").to_lowercase();
    return clean_input_string == clean_input_string.chars().rev().collect::<String>();
}


// Achar número array
fn search_in_array(array: [i8; 5], target: i8) -> bool {
    for value in array.iter() {
        if *value == target {
            return true
        }
    } return false
}


// Contar pares em array
fn search_pairs (array: [i8; 5], target: i8) -> i8 {
    let mut target_repetitions: i8 = 0;
    for value in array.iter() {
        if *value == target {
            target_repetitions += 1;
        }
    }
    return target_repetitions;
}


#[cfg(test)]
mod check_palindrome{
    use crate::*;
    
    #[test]
    fn is_palindrome() {
        let my_string: &str = "Arara";
        assert_eq!(true, compare_strings(my_string));
    }
    
    #[test]
    fn is_in_array () {
        let reference_array: [i8; 5] = [83, -54, 127, 36, -79];
        let search_attempt: i8 = 36;
        assert_eq!(true, search_in_array(reference_array, search_attempt));
    }
    
    #[test]
    fn pairs_in_array () {
        let reference_array: [i8; 5] = [8, 5, 8, 3, 3];
        let search_attempt: i8 = 8;
        assert_eq!( 2, search_pairs(reference_array, search_attempt));
    }
}
