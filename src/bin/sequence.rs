fn count_divisors(mut n: u64) -> u64 {
    let mut divisors = 1;
    let mut p = 2;

    while p * p <= n {
        if n % p == 0 {
            let mut exponent = 0;
            while n % p == 0 {
                n /= p;
                exponent += 1;
            }
            divisors *= exponent + 1;
        }
        p += if p == 2 { 1 } else { 2 }; 
    }

    if n > 1 {
        divisors *= 2; 
    }

    divisors
}

fn main() {
    let mut n = 1u64;
    loop {
        let (a, b) = if n % 2 == 0 {
            (n / 2, n + 1)
        } else {
            (n, (n + 1) / 2)
        };

        let divisors = count_divisors(a) * count_divisors(b);

        if divisors >= 500 {
            let triangle = n * (n + 1) / 2;
            println!("Triangle = {}, divisors = {}", triangle, divisors);
            break;
        }

        n += 1;
    }
}

