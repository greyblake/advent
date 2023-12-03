fn main() {
    let input = include_str!("input.txt");
    let sum = compute(input);
    println!("Sum: {}", sum);
}

fn compute(input: &str) -> u32 {
    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line_for_numbers(y, line))
        .collect();

    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| parse_line_for_symbols(y, line))
        .collect();

    numbers
        .iter()
        .filter(|n| is_adjusted_to_one_of_symbols(n, &symbols))
        .map(|n| n.value)
        .sum::<u32>()
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Number {
    value: u32,
    pos: Pos,
    size: usize,
}

impl Number {
    fn all_taken_positions(&self) -> Vec<Pos> {
        let mut positions = Vec::with_capacity(self.size);

        for x in self.pos.x..(self.pos.x + self.size) {
            positions.push(Pos { x, y: self.pos.y });
        }

        positions
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Symbol {
    value: char,
    pos: Pos,
}

impl Symbol {
    // All the 8 positions around the symbol
    fn all_nearby_positions(&self) -> Vec<Pos> {
        let mut positions = Vec::with_capacity(8);

        let minx = if self.pos.x == 0 { 0 } else { self.pos.x - 1 };
        let maxx = self.pos.x + 1;
        let miny = if self.pos.y == 0 { 0 } else { self.pos.y - 1 };
        let maxy = self.pos.y + 1;

        for x in minx..=maxx {
            for y in miny..=maxy {
                if x == self.pos.x && y == self.pos.y {
                    continue;
                }
                positions.push(Pos { x, y });
            }
        }

        positions
    }
}

fn is_adjusted_to_one_of_symbols(number: &Number, symbols: &[Symbol]) -> bool {
    let number_positions = number.all_taken_positions();
    let symbol_positions: Vec<Pos> = symbols
        .iter()
        .flat_map(|s| s.all_nearby_positions())
        .collect();
    for number_pos in number_positions.iter() {
        if symbol_positions.contains(&number_pos) {
            return true;
        }
    }
    false
}

// TODO: refactor
// * Use better types for state
// * DRY termination of number
fn parse_line_for_numbers(y: usize, line: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    let mut buf = String::with_capacity(4);
    let mut start_x = None;

    line.chars().enumerate().for_each(|(x, c)| match c {
        '0'..='9' => {
            buf.push(c);
            if start_x.is_none() {
                start_x = Some(x);
            }
        }
        _ => {
            if !buf.is_empty() {
                let number = buf.parse::<u32>().unwrap();
                numbers.push(Number {
                    value: number,
                    pos: Pos {
                        x: start_x.unwrap(),
                        y,
                    },
                    size: buf.len(),
                });

                buf.clear();
                start_x = None;
            }
        }
    });

    if !buf.is_empty() {
        let number = buf.parse::<u32>().unwrap();
        numbers.push(Number {
            value: number,
            pos: Pos {
                x: start_x.unwrap(),
                y,
            },
            size: buf.len(),
        });

        buf.clear();
        start_x = None;
    }

    numbers
}

// TODO: combine it with parse_line_for_numbers()
fn parse_line_for_symbols(y: usize, line: &str) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();

    line.chars().enumerate().for_each(|(x, c)| match c {
        '0'..='9' | '.' => { },
        ch => {
            symbols.push(Symbol {
                value: ch,
                pos: Pos { x, y },
            });
        }
        _ => {}
    });

    symbols
}

#[test]
fn should_parse_line_for_numbers() {
    let line = "..35..633";
    let numbers = parse_line_for_numbers(15, line);

    assert_eq!(numbers.len(), 2);
    assert_eq!(
        numbers[0],
        Number {
            value: 35,
            pos: Pos { x: 2, y: 15 },
            size: 2,
        }
    );
    assert_eq!(
        numbers[1],
        Number {
            value: 633,
            pos: Pos { x: 6, y: 15 },
            size: 3,
        }
    );
}

#[test]
fn should_parse_line_for_symbols() {
    let line = "617*.$.";
    let symbols = parse_line_for_symbols(0, line);

    assert_eq!(symbols.len(), 2);
    assert_eq!(
        symbols[0],
        Symbol {
            value: '*',
            pos: Pos { x: 3, y: 0 },
        }
    );
    assert_eq!(
        symbols[1],
        Symbol {
            value: '$',
            pos: Pos { x: 5, y: 0 },
        }
    );
}

#[test]
fn should_compute() {
    // Input from the example in the task
    let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
    .trim();
    let sum = compute(input);
    assert_eq!(sum, 4361);
}
