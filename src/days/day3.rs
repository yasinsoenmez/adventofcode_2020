use std::path::Path;

pub fn get_input() -> String {
    let path = Path::new("input/day3.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn day3() -> (u32, u32) {
    let input = get_input();

    (day3_1(&input), day3_2(&input))
}

pub fn day3_1(input: &str) -> u32 {
    trees_on_slope(input, 3, 1)
}

pub fn day3_2(input: &str) -> u32 {
    trees_on_slope(input, 1, 1) *
    trees_on_slope(input, 3, 1) *
    trees_on_slope(input, 5, 1) *
    trees_on_slope(input, 7, 1) *
    trees_on_slope(input, 1, 2)
}

fn trees_on_slope(input: &str, right: usize, down: usize) -> u32 {
    let mut x = 0;

    input.lines()
         .step_by(down)
         .filter_map(|s| {
             let tree = match s.trim().chars().nth(x).unwrap() == '#' {
                 true => Some(true),
                 false => None,
             };
             x = (x + right) % s.trim().len();
             tree
         })
         .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_test() {
        let (day3_1, day3_2) = day3();
        assert_eq!(day3_1, 220); //220
        assert_eq!(day3_2, 2138320800); //2138320800
    }

    #[test]
    fn day3_extra() {
        let input = String::from("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#");

        assert_eq!(day3_1(&input), 7); // 7
        assert_eq!(day3_2(&input), 336); // 336
    }
}