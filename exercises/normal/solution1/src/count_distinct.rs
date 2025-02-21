use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut unique_elem = HashSet::new();

    for elem in input_str.split(','){
        unique_elem.insert(elem);
    }
    unique_elem.len()
}
