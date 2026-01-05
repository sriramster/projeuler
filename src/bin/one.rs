const MAX_NUM: usize = 1000;

fn main() {
    let mut sum: usize = 0;
    for i in 1..=MAX_NUM - 1 {
        if i % 3 == 0 || i % 5 == 0 {
            println!("{i}");
            sum += i;
        }
    }
    println!("{sum}")
}
