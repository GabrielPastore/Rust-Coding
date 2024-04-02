use std::collections::HashMap;

fn check_pairs (array: [i8; 5]) -> HashMap<i8, u8> {
    let mut counter: HashMap<i8, u8> = HashMap::new();

    for instance in array {
        *counter.entry(instance).or_insert(0) += 1;
    }
    for (_, repetitions) in counter.iter_mut() {
        if *repetitions > 0 {
            *repetitions /= 2;
        }
    }
    return counter
}

#[cfg(test)]
mod search_in_array {
    use crate::*;
   
    #[test]
    fn numeric_order_array () {
        let reference_array: [i8; 5] = [1,1,2,3,3];
        let repetitions = check_pairs(reference_array);
        assert_eq!(repetitions[&1], 1);
        assert_eq!(repetitions[&3], 1);
    }

    #[test]
    fn non_numeric_order_array () {
        let reference_array: [i8; 5] = [8,5,8,4,5];
        let repetitions = check_pairs(reference_array);
        assert_eq!(repetitions[&8], 1);
        assert_eq!(repetitions[&5], 1);
    }
    
    #[test]
    fn no_pairs_array () {
        let reference_array: [i8; 5] = [8,5,9,7,4];
        let repetitions = check_pairs(reference_array);
        assert_eq!(repetitions[&8], 0);
        assert_eq!(repetitions[&9], 0);
    }
}
