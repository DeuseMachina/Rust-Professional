pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut ret = 1;

    while number % 2 == 0{
        ret = 2;
        number /= 2;
    }

    let mut cur = 3;
    while cur * cur <= number{
        while number % cur == 0{
            ret = cur;
            number /= cur;
        }
        cur += 2;
    }
    if number > ret {
        ret = number;
    }
    ret
}
