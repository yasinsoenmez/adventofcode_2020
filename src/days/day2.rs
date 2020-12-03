use std::path::Path;

pub struct Pass {
    min: u32,
    max: u32,
    character: char,
    password: String
}

pub fn day2() -> (u32, u32) {
    let input = get_input();
    let input = string2vec(input.as_str());

    (day2_1(&input), day2_2(&input))
}

pub fn day2_1(input: &Vec<Pass>) -> u32 {
    input.iter()
        .filter(|pass| {
            let count = pass.password
                .chars()
                .filter(|c| c == &pass.character)
                .count() as u32;
            pass.min <= count && pass.max >= count
        })
        .count() as u32
}

pub fn day2_1_string(input: &str) -> u32 {
    input.lines()
         .filter(|pass| {
             let min_rest = pass.split('-').collect::<Vec<&str>>();
             let max_rest_pass = min_rest[1].split(' ').collect::<Vec<&str>>();
             let count = max_rest_pass[2].chars()
                .filter(|c| c == &max_rest_pass[1].chars().nth(0).unwrap())
                .count() as u32;
             min_rest[0].parse::<u32>().unwrap() <= count && max_rest_pass[0].parse::<u32>().unwrap() >= count
         })
         .count() as u32
}

pub fn day2_2(input: &Vec<Pass>) -> u32 {
    input.iter()
        .filter(|pass| {
            (pass.password.chars().nth((pass.min - 1) as usize) == Some(pass.character))
                ^ (pass.password.chars().nth((pass.max - 1) as usize) == Some(pass.character))
        })
        .count() as u32
}

pub fn get_input() -> String {
    let path = Path::new("input/day2.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn string2vec(input: &str) -> Vec<Pass> {
    input.lines()
         .map(|s| {
             let min_rest = s.split('-').collect::<Vec<&str>>();
             let max_rest_pass = min_rest[1].split(' ').collect::<Vec<&str>>();
             Pass {
                 min: min_rest[0].parse().unwrap(),
                 max: max_rest_pass[0].parse().unwrap(),
                 character: max_rest_pass[1].chars().nth(0).unwrap(),
                 password: max_rest_pass[2].to_string()
             }
         }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test() {
        let (day2_1, day2_2) = day2();
        assert_eq!(day2_1, 393); //393
        assert_eq!(day2_2, 690); //690
    }

    #[test]
    fn day2_extra() {
        let input = String::from("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        let input = string2vec(&input);

        assert_eq!(day2_1(&input), 2); // 514579
        assert_eq!(day2_2(&input), 1); // 241861950
    }
}