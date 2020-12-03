use std::path::Path;
use std::collections::HashMap;

pub fn day1() -> (i32, i32) {
    let numbers = input2hash(&get_input());

    (day1_1(&numbers), day1_2(&numbers))
}

pub fn day1_1(input: &HashMap<i32, i32>) -> i32 {
    input.iter()
         .filter_map(|(_, value)| {
             input.get_key_value(value)
         })
         .next()
         .map_or_else(|| 0, |(key, value)| key * value)
}

pub fn day1_1_alt(input: &str) -> i32 {
    input.lines()
        .filter_map(|s| {
            let first = s.parse::<i32>().unwrap();
            let second = 2020 - first;
            match input.contains(second.to_string().as_str()) {
                true => Some(first * second),
                false => None
            }
        })
        .next()
        .unwrap()
}

pub fn day1_2(input: &HashMap<i32, i32>) -> i32 {
    input.iter()
         .enumerate()
         .filter_map(|(counter, (key1, _))| {
             input.iter()
                  .skip(counter + 1)
                  .filter_map(|(key2, _)| {
                      input.get_key_value(&(2020 - key1 - key2))
                  })
                  .next()
                  .map_or_else(|| None, |(key, _)|  Some(key * key1 * (2020 - key - key1)))
         })
        .next()
        .unwrap_or_else(|| 0)
}

pub fn get_input() -> String {
    let path = Path::new("input/day1.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn input2hash(input: &String) -> HashMap<i32, i32> {
    input.lines()
        .map(|s| {
            let i = s.parse::<i32>().unwrap();
            (i, 2020 - i)
        })
        .collect()
}

#[warn(dead_code)]
pub fn day1_1_old(input: &Vec<i32>) -> i32 {
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

#[warn(dead_code)]
pub fn day1_2_old(input: &Vec<i32>) -> i32 {
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
        let input = String::from("1721\n979\n366\n299\n675\n1456");
        let input = input2hash(&input);

        assert_eq!(day1_1(&input), 514579); // 514579
        assert_eq!(day1_2(&input), 241861950); // 241861950
    }
}
