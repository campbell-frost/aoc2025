pub fn solve(input: &str) -> i64 {
    input
        .split(',')
        .map(|pair| {
            let (l, r) = pair.split_once('-').unwrap();
            find_invalid_ids(get_val(l), get_val(r))
        })
        .flatten()
        .sum()
}

fn get_val(s: &str) -> i64 {
    s.parse().unwrap()
}

fn find_invalid_ids(mut left: i64, right: i64) -> Vec<i64> {
    let mut vec = vec![];
    while left <= right {
        if is_invalid(left) {
            vec.push(left);
        }
        left = left + 1;
    }
    return vec;
}

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // check all possible pattern lengths (can be 1 - len / 2 since we need at least two patterns)
    for pattern_len in 1..=len / 2 {
        // does the pattern evenly divide the string?
        if len.rem_euclid(pattern_len) == 0 {
            let pattern = &s[..pattern_len];

            // if we repeat this pattern to fill the length of the string does it match?
            if pattern.repeat(len / pattern_len) == s {
                return true;
            }
        }
    }

    false
}
