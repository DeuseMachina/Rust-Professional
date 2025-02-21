pub fn new_birthday_probability(n: u32) -> f64 {
    let mut para = 1.0 as f64;
    for i in 0..n{
        para *= (365 - i) as f64 / 365.0;
    }
    1.0 - para
}
