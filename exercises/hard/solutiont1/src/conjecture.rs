fn gen_primes(limit:usize) -> Vec<bool>{
    let mut sieve = vec![true; limit+1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..= ((limit as f64).sqrt()) as usize {
        for j in (i*i..=limit).step_by(i){
            sieve[j] = false;
        }
    }
    sieve
}
pub fn goldbach_conjecture() -> String {
    let primes = gen_primes(1000000);
    let mut ret = Vec::new();
    let mut current = 9;
    while ret.len() < 2{
        if !primes[current as usize]{
            let mut flag = false;
            for i in 1..{
                let square = i * i;
                if square >= current{
                    break;
                }
                let diff = current - 2 * square;
                if diff > 0 && primes[diff as usize]{
                    flag = true;
                    break;
                }
            }
            if !flag{
                ret.push(current);
            }
        }
        current += 2;
    }
    format!("{},{}", ret[0], ret[1])
}
