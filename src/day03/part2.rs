pub fn solve(input: &str) -> i64 {
    input
        .lines()
        .map(|bank| {
            let batteries: Vec<i8> = bank
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect();

            let best = find_best_k_batteries(&batteries, 12);
            joltage_from_digits(&best)
        })
        .sum()
}

// to find the best batteries we can traverse left to right like we did in part 1
// and we can use a stack to keep track of the highest numbers we encounter in order
fn find_best_k_batteries(batteries: &[i8], k: usize) -> Vec<i8> {
    let mut stack: Vec<i8> = Vec::with_capacity(k);

    for (i, &d) in batteries.iter().enumerate() {
        let remaining = batteries.len() - i;

        while let Some(&last) = stack.last() {
            if d > last && stack.len() - 1 + remaining >= k {
                // we can pop the stack if the current battery is larger than what is currently
                // on top of the stack AND we can still fill the stack with the remaining numbers

                stack.pop();
            } else {
                break;
            }
        }

        // if we have not filled up the stack, push the current digit
        if stack.len() < k {
            stack.push(d);
        }
    }

    stack
}

fn joltage_from_digits(selected: &[i8]) -> i64 {
    let mut out: i64 = 0;
    for &d in selected {
        out = out * 10 + (d as i64);
    }
    out
}
