pub fn solve(input: &str) -> i16 {
    input
        .lines()
        .map(|bank| {
            get_joltage(
                &bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i16)
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn get_joltage(batteries: &[i16]) -> i16 {
    let mut max_val: i16 = -1;

    // traverse the entire array with two pointers, with i starting at 0
    // and j starting at i + 1 and find the max value that can be created
    // where j > i
    for i in 0..batteries.len() {
        for j in (i + 1)..batteries.len() {
            let a = batteries[i] as i16;
            let b = batteries[j] as i16;
            let value = a * 10 + b;
            if value > max_val {
                max_val = value;
            }
        }
    }

    max_val
}
