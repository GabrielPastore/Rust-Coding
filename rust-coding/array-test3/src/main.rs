#[derive(PartialEq, Debug)]
struct Pairs {
    have_pair: i8,
    number_of_pairs: i8,
}

fn count_pairs(array: [i8; 5]) -> Vec<Pairs> {
    let mut repetition_counter: i8 = 1;
    let mut dif: i8 = array[0] - 0;
    let mut pairs_list: Vec<Pairs> = Vec::new();

    for index in 0..array.len() {
        let instance: i8 = array[index]; 

        if instance - dif != index.try_into().unwrap() {
            repetition_counter += 1;
            dif -= 1;
            if array.len()-1 != index.try_into().unwrap() {
                if instance != array[index+1] {
                    let pair_found = Pairs {
                        have_pair: instance,
                        number_of_pairs: repetition_counter/2,
                    };
                    repetition_counter = 1;
                    pairs_list.push(pair_found);
                }
            } else if instance == array[index-1] {
                let pair_found = Pairs {
                        have_pair: instance,
                        number_of_pairs: repetition_counter/2,
                    };
                    pairs_list.push(pair_found);
            }
        }
    }
    return pairs_list;
}

#[cfg(test)]
mod search_in_array {
    use crate::*;
    #[test]
    fn pairs_in_array() {
        let reference_array: [i8; 5] = [1,1,2,3,3];

        let pair1 = Pairs {
          have_pair: 1,
          number_of_pairs: 1,
        };
        let pair2 = Pairs {
          have_pair: 3,
          number_of_pairs: 1,
        };

        let test_pairs_list: Vec<Pairs> = vec![pair1, pair2];
        assert_eq!(test_pairs_list, count_pairs(reference_array));
    }
}
