mod part1;

pub fn run() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("pt1: {}", part1::solve(&input));
}
