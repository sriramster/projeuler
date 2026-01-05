const MAX_NUM: usize = 529_000;

fn main() {
    let mut sum: usize = 0;
    for i in 1..=MAX_NUM {
        if i % 2 != 0 {
            sum += i * i
        }
    }
    println!("{sum}")
}
