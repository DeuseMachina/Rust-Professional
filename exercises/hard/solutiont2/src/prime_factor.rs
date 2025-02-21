/// 计算两个数的最大公约数（GCD）
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Pollard's Rho 因数分解算法
fn pollard_rho(n: u128) -> u128 {
    // 处理基本情况
    if n == 1 { return 1; }
    if n % 2 == 0 { return 2; }
    if miller_rabin(n) { return n; }

    // 定义伪随机函数 f(x) = (x² + 1) mod n
    let f = |x: u128| (x.wrapping_mul(x) + 1) % n;

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;

    // Floyd 循环检测
    while d == 1 {
        x = f(x);       // 慢指针步进1次
        y = f(f(y));    // 快指针步进2次
        d = gcd(x.abs_diff(y), n);
    }

    // 递归分解因数
    if d == n {
        pollard_rho(n)  // 避免无限循环：随机种子变化后重试
    } else {
        let factor1 = pollard_rho(d);
        let factor2 = pollard_rho(n / d);
        factor1.max(factor2)
    }
}

/// Miller-Rabin 素数检测（确定性版本，适用于 n < 2^64）
fn miller_rabin(n: u128) -> bool {
    // 处理小数字
    match n {
        0 | 1 => return false,
        2 | 3 => return true,
        _ if n % 2 == 0 => return false,
        _ => {}
    }

    // 预选测试基（覆盖 2^64 范围）
    const BASES: [u128; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    
    // 分解 n-1 = d*2^s
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // 对每个基进行测试
    'base_loop: for &a in &BASES {
        let a = a % n;
        if a == 0 { continue; }
        
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 { continue; }

        // 平方探测
        for _ in 0..s-1 {
            x = mod_pow(x, 2, n);
            if x == n - 1 { continue 'base_loop; }
        }
        return false;
    }
    true
}

/// 快速幂取模算法 (base^exp % modulus)
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0; }
    let mut result: u128 = 1;
    base %= modulus;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    result
}

/// 查找最大质因数（优化版本）
pub fn find_max_prime_factor(mut n: u128) -> u128 {
    // 处理边界情况
    if n <= 1 { return n; }
    
    // 预生成小质数列表（前168个质数，覆盖到1000）
    const SMALL_PRIMES: [u128; 46] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
        73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
        157, 163, 167, 173, 179, 181, 191, 193, 197, 199
    ];
    
    let mut max_factor = 1;

    // 快速去除小质因数
    for &p in &SMALL_PRIMES {
        if p > n { break; }
        while n % p == 0 {
            max_factor = max_factor.max(p);
            n /= p;
        }
        if n == 1 { return max_factor; }
    }

    // 处理剩余的大数
    if n > 1 {
        let big_factor = pollard_rho(n);
        max_factor = max_factor.max(big_factor);
    }

    max_factor
}