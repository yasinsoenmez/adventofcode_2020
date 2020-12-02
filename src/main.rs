pub mod days;
use days::day1;

fn main() {
    let (day1_1, day1_2) = day1::day1();
    println!("Day1_1: {}", day1_1);
    println!("Day1_2: {}\n", day1_2);
}
