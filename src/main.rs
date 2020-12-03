pub mod days;
use days::day1;
use days::day2;
use days::day3;
use days::day4;

fn main() {
    let (day1_1, day1_2) = day1::day1();
    println!("Day1_1: {}", day1_1);
    println!("Day1_2: {}\n", day1_2);

    let (day2_1, day2_2) = day2::day2();
    println!("Day2_1: {}", day2_1);
    println!("Day2_2: {}\n", day2_2);

    let (day3_1, day3_2) = day3::day3();
    println!("Day3_1: {}", day3_1);
    println!("Day3_2: {}\n", day3_2);

    let (day4_1, day4_2) = day4::day4();
    println!("Day4_1: {}", day4_1);
    println!("Day4_2: {}\n", day4_2);
}
