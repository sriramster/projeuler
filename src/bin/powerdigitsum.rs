use std::env;
use num_bigint::BigUint;
use num_traits::One;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} exponent", args[0]);
        std::process::exit(1);
    }

    let exp: u32 = args[1].parse()?;

    let two = BigUint::from(2u32);
    let val = two.pow(exp);

    let sum: u32 = val
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    println!("{}", sum);
    Ok(())
}
