pub fn solve(input: &str) -> i32 {
    let mut current_pos = 50;
    let mut zero_count = 0;

    let rows = input.lines();

    for row in rows {
        let (direction, value) = row.split_at(1);
        let parsed_value: i32 = value.parse().unwrap();

        // get absolute value to add to current_pos
        let signed = if direction == "L" {
            -parsed_value
        } else {
            parsed_value
        };

        // find out how many times signed will cross over 0
        let crossed = get_crossed_count(current_pos, signed);
        zero_count += crossed;

        // update the current pos to new position
        let turn_value = signed.rem_euclid(100);

        current_pos = (current_pos + turn_value).rem_euclid(100);
    }

    zero_count
}

fn get_crossed_count(current_pos: i32, signed: i32) -> i32 {
    // how many times are we moving around the dial?
    let steps = signed.abs();

    // get the amount of steps needed to reach 0
    let first_hit_dist = if signed > 0 {
        if current_pos == 0 {
            100
        } else {
            100 - current_pos
        }
    } else if signed < 0 {
        if current_pos == 0 { 100 } else { current_pos }
    } else {
        return 0;
    };

    // if we wont cross, just return 0
    if steps < first_hit_dist {
        return 0;
    }

    // return number of croses
    1 + (steps - first_hit_dist) / 100
}
