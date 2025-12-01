pub fn solve(input: &str) -> u16 {
    let mut tracked_pos = 50;
    let mut zero_count: u16 = 0;

    let rows = input.lines();

    for row in rows {
        let (k, v) = row.split_at(1);
        let parsed_val: i32 = v.parse().unwrap();
        let turn_value = get_turn_value(k, parsed_val);

        tracked_pos = (tracked_pos + turn_value).rem_euclid(100);

        if tracked_pos == 0 {
            zero_count = zero_count + 1;
        }
    }

    return zero_count;
}

fn get_turn_value(direction: &str, val: i32) -> i32 {
    let delta = val.rem_euclid(100);
    if direction == "L" {
        return -delta;
    } else {
        return delta;
    }
}
