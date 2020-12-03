use std::path::Path;

pub fn get_input() -> String {
    let path = Path::new("input/day4.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn day4() -> (u32, u32) {
    let input = get_input();

    (day4_1(&input), day4_2(&input))
}

pub fn day4_1(input: &str) -> u32 {
    0
}

pub fn day4_2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_test() {
        let (day4_1, day4_2) = day4();
        assert_eq!(day4_1, 0); //
        assert_eq!(day4_2, 0); //
    }

    #[test]
    fn day4_extra() {
        let input = String::from("");

        assert_eq!(day4_1(&input), 0); //
        assert_eq!(day4_2(&input), 0); //
    }
}
