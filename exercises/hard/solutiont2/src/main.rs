// I AM NOT DONE

mod prime_factor;

fn main() {
    let number = 199999999999999951437;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");
}
