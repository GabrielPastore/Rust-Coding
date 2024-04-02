fn search_number(array: [i8; 5], target: i8) -> bool {
    let half: i8 = (array[0] + array[array.len()-1]) / 2;
    
    if target > half {
        for instance in &array[half as usize..array.len()] {
            if *instance == target {
                return true
            } 
        }
    } else if target < half {
        for instance in &array[0..half as usize] {
            if *instance == target {
                return true
            }
        }
    } else if target == half {
        return true
    } 

    return false
}

#[cfg(test)]
mod search_number_in_array {
    use crate::*;

    #[test]
    fn bigger_than_half () {
        let reference_array: [i8; 5] = [1,2,3,4,5];
        assert_eq!(true, search_number(reference_array, 5));
    }

    #[test]
    fn smaller_than_half () {
        let reference_array: [i8; 5] = [1,2,3,4,5];
        assert_eq!(true, search_number(reference_array, 2));
    }

    #[test]
    fn exactly_half () {
        let reference_array: [i8; 5] = [1,2,3,4,5];
        assert_eq!(true, search_number(reference_array, 3));
    }
    
    #[test]
    fn not_in_array () {
        let reference_array: [i8; 5] = [1,2,3,4,5];
        assert_eq!(false, search_number(reference_array, 7));
    }
}
