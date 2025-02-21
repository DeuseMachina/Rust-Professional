// src/converter.rs
use std::str::FromStr;

pub fn convert_base(input: &str, target_base: u32) -> String {
    let parts: Vec<&str> = input.split('(').collect();
    if parts.len() != 2 || !parts[1].ends_with(')') {
        return "Invalid input format".to_string();
    }
    let number_str = parts[0];
    let original_base_str = &parts[1][..parts[1].len() - 1]; 
    let original_base = match u32::from_str(original_base_str) {
        Ok(val) => val,
        Err(_) => return "Invalid base format".to_string(),
    };

    let decimal_number = match i64::from_str_radix(number_str, original_base) {
        Ok(val) => val,
        Err(_) => return "Invalid number for the given base".to_string(),
    };


    if target_base == 10 {
        return decimal_number.to_string();
    }

    let mut n = decimal_number;
    let mut result = Vec::new();

    while n > 0 {
        let remainder = (n % target_base as i64) as u32;
        if remainder < 10 {
            result.push((b'0' + remainder as u8) as char);
        } else {
            result.push((b'a' + (remainder - 10) as u8) as char);
        }
        n /= target_base as i64;
    }

    if result.is_empty() {
        return "0".to_string();
    }

    result.reverse();
    result.into_iter().collect()
}