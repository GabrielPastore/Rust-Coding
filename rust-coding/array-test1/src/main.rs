fn search_in_array(value: [i8; 5], target: i8) -> bool {

}

#[cfg(test)]
mod find_number_in_array {
    use crate::*;
    #[test]
    fn is_in_array () {
        let reference_array: [i8; 5] = [83, -54, 127, 36, -79];
        let search_attempt: i8 = 36;
        assert_eq!(true, search_in_array(reference_array, search_attempt));
    }
}