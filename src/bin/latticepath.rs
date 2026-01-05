use std::env;
use std::error::Error;

fn binomial_2n_n(n: u64) -> u128 {
    let mut result: u128 = 1;

    for i in 1..=n {
        result = result * (n + i) as u128 / i as u128;
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <grid_size>", args[0]);
        std::process::exit(1);
    }

    let n: u64 = args[1].parse()?;
    let res = binomial_2n_n(n);

    println!("{}", res);
    Ok(())
}
