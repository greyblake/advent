fn main() {
    let input = include_str!("input.txt").trim();
    let sum = input.lines()
        .map(line_to_calibration_value)
        .sum::<i32>();
    println!("Sum: {sum}");
}

fn line_to_calibration_value(line: &str) -> i32 {
    let first_digit = get_first_digit(line);
    let last_digit = get_last_digit(line);
    format!("{first_digit}{last_digit}")
        .parse::<i32>()
        .expect("Failed to parse")
}

fn get_first_digit(line: &str) -> char {
    line.chars()
        .find(|c| c.is_digit(10))
        .expect("Failed to get first digit")
}

fn get_last_digit(line: &str) -> char {
    line.chars()
        .rev()
        .find(|c| c.is_digit(10))
        .expect("Failed to get last digit")
}
