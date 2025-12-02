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
    let str = n.to_string();
    let len = str.len();
    // if the number of digits is odd, it cant be invalid
    if len.rem_euclid(2) != 0 {
        return false;
    }
    // if we break it appart into two pieces, the two sides should be equal
    let (left, right) = str.split_at(len / 2);
    if left == right {
        return true;
    }
    false
}
