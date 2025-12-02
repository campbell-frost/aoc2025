mod part1;
mod part2;

pub fn run() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();
    println!("pt1: {}", part1::solve(&input));
    println!("pt2: {}", part2::solve(&input));
}
