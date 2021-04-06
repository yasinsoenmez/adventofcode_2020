use std::{cmp::Ordering, path::Path};

#[derive(Debug, Eq, PartialOrd)]
pub struct BoardingPass {
    seat_id: u32,
    row: u32,
    col: u32,
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seat_id.cmp(&other.seat_id)
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.seat_id == other.seat_id
    }
}

impl From<&str> for BoardingPass {
    fn from(input: &str) -> Self {
        let mut row: u32 = 0;
        let mut col: u32 = 0;

        for c in input.chars() {
            match c {
            'B' => row = (row << 1) + 1,
            'F' => row = row << 1,
            'R' => col = (col << 1) + 1,
            'L' => col = col << 1,
             _  => println!("Something went wrong converting the data."),
            }
        }

        BoardingPass {
            seat_id: row * 8 + col,
            row,
            col,
        }
    }
}

pub fn get_input() -> String {
    let path = Path::new("input/day5.txt");
    std::fs::read_to_string(&path).expect("Couldn't read the input")
}

pub fn convert(input: &str) -> Vec<BoardingPass> {
    let mut passes = input.lines()
                          .map(|s| {
                              BoardingPass::from(s)
                          })
                          .collect::<Vec<BoardingPass>>();
    passes.sort_unstable();
    passes
}

pub fn day5() -> (u32, u32) {
    let input = get_input();
    let input = convert(&input);

    (day5_1(&input), day5_2(&input))
}

pub fn day5_1(boarding_passes: &Vec<BoardingPass>) -> u32 {
    boarding_passes.last()
                   .unwrap()
                   .seat_id
}

pub fn day5_2(boarding_passes: &Vec<BoardingPass>) -> u32 {
    let bp_offset = boarding_passes.first().unwrap().seat_id;

    boarding_passes.iter()
                   .enumerate()
                   .find(|(i, b)| *i != (b.seat_id - bp_offset) as usize)
                   .unwrap()
                   .0 as u32 + bp_offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_test() {
        let (day5_1, day5_2) = day5();
        assert_eq!(day5_1, 953); // 953
        assert_eq!(day5_2, 615); // 615
    }

    #[test]
    fn day5_extra() {
        let input1 = BoardingPass::from("BFFFBBFRRR");
        let input2 = BoardingPass::from("FFFBBBFRRR");
        let input3 = BoardingPass::from("BBFFBBFRLL");

        assert_eq!(day5_1(&vec![input1]), 567); // 567
        assert_eq!(day5_1(&vec![input2]), 119); // 119
        assert_eq!(day5_1(&vec![input3]), 820); // 820
    }
}