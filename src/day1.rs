use std::path::Path;

pub fn day1() -> (i32, i32) {
    let path = Path::new("input/day1.txt");
    let input = std::fs::read_to_string(&path).expect("Couldn't read the input");
    let numbers = input.lines()
                       .map(|s| s.parse::<i32>().unwrap())
                       .collect::<Vec<i32>>();
    (day1_1(&numbers), day1_2(&numbers))
}

fn day1_1(input: &Vec<i32>) -> i32 {
    let copy = &input[1..].to_vec();

    for i in input.iter() {
        for j in copy.iter() {
            if i + j == 2020 {
                return i * j
            }
        }
    }
    0
}

fn day1_2(input: &Vec<i32>) -> i32 {
    let copy = &input[1..].to_vec();
    let copy2 = &input[2..].to_vec();

    for i in input.iter() {
        for j in copy.iter() {
            for k in copy2.iter() {
                if i + j + k == 2020 {
                    return i * j * k
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_test() {
        let (day1_1, day1_2) = day1();
        assert_eq!(day1_1, 927684); //927684
        assert_eq!(day1_2, 292093004); //927684
    }
    #[test]
    fn day1_extra() {
        assert_eq!(day1_1(&vec![1721, 979, 366, 299, 675, 1456]), 514579); // 514579
        assert_eq!(day1_2(&vec![1721, 979, 366, 299, 675, 1456]), 241861950); // 241861950
    }
}