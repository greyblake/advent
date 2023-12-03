fn main() {
    let input = include_str!("input.txt");
    let sum = compute(input);
    println!("Sum: {}", sum);
}

fn compute(input: &str) -> u32 {
    let (numbers, symbols) = parsing::parse(input);

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

mod parsing {
    use super::*;

    pub fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        let mut state = StateMachine::default();

        let mut handle_output = |output: Option<Output>| match output {
            Some(Output::Number(number)) => numbers.push(number),
            Some(Output::Symbol(symbol)) => symbols.push(symbol),
            None => {}
        };

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let out = state.feed_char(x, y, ch);
                handle_output(out);
            }
            handle_output(state.flush());
        }
        handle_output(state.flush());

        (numbers, symbols)
    }

    #[derive(Debug, Default)]
    enum StateMachine {
        #[default]
        Empty,
        Number(Pos, String),
        Symbol(Pos, char),
    }

    enum Output {
        Number(Number),
        Symbol(Symbol),
    }

    impl StateMachine {
        fn feed_char(&mut self, x: usize, y: usize, input_char: char) -> Option<Output> {
            let current_state = std::mem::take(self);

            let (new_state, res) = match current_state {
                StateMachine::Empty => {
                    match input_char {
                        '0'..='9' => {
                            // Start a new number
                            let pos = Pos { x, y };
                            let s = String::from(input_char);
                            (StateMachine::Number(pos, s), None)
                        }
                        '.' => {
                            // Do nothing, skip
                            (StateMachine::Empty, None)
                        }
                        sym => {
                            // Put a new symbol to the state
                            let pos = Pos { x, y };
                            (StateMachine::Symbol(pos, sym), None)
                        }
                    }
                }
                StateMachine::Number(num_pos, mut num_str) => {
                    match input_char {
                        '0'..='9' => {
                            // Attach a new digit to the current number
                            num_str.push(input_char);
                            (StateMachine::Number(num_pos, num_str), None)
                        }
                        '.' => {
                            // Flush the current number and set empty state
                            (StateMachine::Empty, output_number(num_pos, num_str))
                        }
                        sym => {
                            let sym_pos = Pos { x, y };
                            (
                                StateMachine::Symbol(sym_pos, sym),
                                output_number(num_pos, num_str),
                            )
                        }
                    }
                }
                StateMachine::Symbol(sym_pos, sym_ch) => {
                    match input_char {
                        '0'..='9' => {
                            // Start a new number
                            let pos = Pos { x, y };
                            let s = String::from(input_char);
                            (StateMachine::Number(pos, s), output_symbol(sym_pos, sym_ch))
                        }
                        '.' => (StateMachine::Empty, output_symbol(sym_pos, sym_ch)),
                        sym => {
                            let pos = Pos { x, y };
                            let new_state = StateMachine::Symbol(pos, sym);
                            (new_state, output_symbol(sym_pos, sym_ch))
                        }
                    }
                }
            };

            *self = new_state;
            res
        }

        fn flush(&mut self) -> Option<Output> {
            let state = std::mem::take(self);
            match state {
                StateMachine::Empty => None,
                StateMachine::Symbol(pos, value) => output_symbol(pos, value),
                StateMachine::Number(pos, s) => output_number(pos, s),
            }
        }
    }

    fn output_number(pos: Pos, s: String) -> Option<Output> {
        let size = s.len();
        let value: u32 = s.parse().expect("Invalid number");
        let number = Number { pos, size, value };
        Some(Output::Number(number))
    }

    fn output_symbol(pos: Pos, value: char) -> Option<Output> {
        Some(Output::Symbol(Symbol { pos, value }))
    }
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
