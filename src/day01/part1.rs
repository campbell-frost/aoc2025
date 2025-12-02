pub fn solve(input: &str) -> u16 {
    let mut current_pos = 50;
    let mut zero_count = 0;

    let rows = input.lines();

    for row in rows {
        let (direction, value) = row.split_at(1);
        let parsed_value: i32 = value.parse().unwrap();
        let turn_value = get_turn_value(direction, parsed_value);

        current_pos = (current_pos + turn_value).rem_euclid(100);

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn get_turn_value(direction: &str, val: i32) -> i32 {
    let delta = val.rem_euclid(100);
    if direction == "L" { -delta } else { delta }
}
