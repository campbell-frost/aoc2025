mod part1;

pub fn run() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();
    println!("{}", part1::solve(&input));
}