pub fn input2vec(input: &String) -> Vec<i32> {
    input.lines()
        .map(|s| { s.parse::<i32>().unwrap() })
        .collect()
}