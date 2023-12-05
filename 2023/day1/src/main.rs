use regex::Regex;

fn main() {
    let input = include_str!("input.txt").trim();

    let result1 = compute_part1(input);
    println!("Part1: {}", result1);

    let result2 = compute_part2(input);
    println!("Part2: {}", result2);

    let result2 = day_one(input.to_owned());
    println!("Part2: {}", result2);
}

fn compute_part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(line_to_calibration_value)
        .sum()
}

fn compute_part2(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(line_to_calibration_value)
        .sum()
}

fn line_to_calibration_value(line: &str) -> i32 {
    let(first_digit, last_digit) = parse_line(line);
    format!("{first_digit}{last_digit}")
        .parse::<i32>()
        .expect("Failed to parse")
}

fn parse_line(line: &str) -> (i32, i32) {
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let numbers: Vec<i32> = re.find_iter(line).map(|m| {
        match m.as_str() {
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => panic!("Failed to parse"),
        }
    }).collect();

    dbg!(&numbers);

    let first = numbers.first().expect("Failed to get first").clone();
    let last = numbers.last().expect("Failed to get last").clone();
    (first, last)
}

#[test]
fn should_compute_part1() {
    let input =
r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#.trim();

    let result = compute_part1(input);
    assert_eq!(result, 142);
}

#[test]
fn should_compute_part2() {
    let input =
r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#.trim();

    let result = compute_part2(input);
    assert_eq!(result, 281);
}
