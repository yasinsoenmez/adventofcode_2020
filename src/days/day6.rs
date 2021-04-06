use std::{collections::{HashMap, HashSet}, path::Path};

pub struct  CustomsDeclarations {
    group: Vec<Group>,
}

pub struct Group {
    people: Vec<People>,
}

pub struct People {
    answers: HashSet<char>,
}

pub fn get_input() -> String {
    let path = Path::new("input/day6.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn convert(input: &str) -> CustomsDeclarations {
    CustomsDeclarations {
        group:
    }
}

pub fn day6() -> (u32, u32) {
    let input = get_input();

    (day6_1(&input), day6_2(&input))
}

pub fn day6_1(input: &str) -> u32 {
    input.split("\n\n")
         .map(|s| {
             let mut answers : HashMap<char, bool> = HashMap::new();
             for line in s.lines() {
                 for c in line.chars() {
                       answers.insert(c, true);
                 }
             }
             answers.len() as u32
         })
         .sum()
}

pub fn day6_2(input: &str) -> u32 {
    /*input.split("\n\n")
         .map(|s| {
             let mut answers : HashMap<char, bool> = HashMap::new();
             s.lines().collect::<HashSet<&str>>().intersection(other)
             for line in s.lines() {
                 for c in line.chars() {
                       answers.insert(c, true);
                       let iter = sets.iter();
let intersection = iter.next().map(|set| iter.fold(set, |set1, set2| set1 & set2));
                 }
             }
             answers.len() as u32
         })
         .sum()*/
         0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test() {
        let (day6_1, day6_2) = day6();
        assert_eq!(day6_1, 6310); // 6310
        assert_eq!(day6_2, 0); // 0
    }

    #[test]
    fn day6_extra() {
        assert_eq!(day6_1("abcx\nabcy\nabcz"), 6); // 6
        assert_eq!(day6_1("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"), 11); // 11
    }
}