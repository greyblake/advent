fn main() {
    let input = include_str!("input.txt");

    let result1 = compute_part1(input);
    println!("Part1 Result: {}", result1);

    let result2 = compute_part2(input);
    println!("Part2 Result: {}", result2);
}

fn compute_part1(input: &str) -> u32 {
    let total_points = input
        .lines()
        .map(Card::parse)
        .map(|card| card.count_points())
        .sum::<u32>();
    total_points
}

fn compute_part2(input: &str) -> u32 {
    let cards = input.lines().map(Card::parse).collect::<Vec<Card>>();
    let mut cards_count = vec![1; cards.len()];

    for (card_index, card) in cards.iter().enumerate() {
        let copies_count = cards_count[card_index];
        let matches = card.count_matches();

        // Add copies for the cards to the next cards
        // TODO: clamp the bounds?
        let from_index = card_index + 1;
        let to_index = card_index + 1 + matches as usize;
        for i in from_index..to_index {
            cards_count[i] += copies_count
        }
    }

    cards_count.iter().sum::<u32>()
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> Card {
        let mut parts = line.split(":");

        let id_part = parts.next().expect("ID part is missing").trim();
        let mut numbers_parts = parts
            .next()
            .expect("Numbers part is missing")
            .trim()
            .split("|");
        let winning_numbers_part = numbers_parts
            .next()
            .expect("Winning numbers part is missing")
            .trim();
        let player_numbers_part = numbers_parts
            .next()
            .expect("Players numbers part is missing")
            .trim();

        let id = id_part
            .chars()
            .filter(|ch| ch.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .expect("Failed to parse Card ID");

        let winning_numbers = winning_numbers_part
            .split(" ")
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| chunk.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let player_numbers = player_numbers_part
            .split(" ")
            .filter(|chunk| !chunk.is_empty())
            .map(|chunk| chunk.trim().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Card {
            id,
            winning_numbers,
            player_numbers,
        }
    }

    fn count_matches(&self) -> u32 {
        let mut matches = 0;
        for player_number in &self.player_numbers {
            if self.winning_numbers.contains(player_number) {
                matches += 1;
            }
        }
        matches
    }

    fn count_points(&self) -> u32 {
        match self.count_matches() {
            0 => 0,
            n => 2u32.pow(n - 1),
        }
    }
}


#[test]
fn should_compute_example() {
    let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    .trim();

    assert_eq!(compute_part1(input), 13);
    assert_eq!(compute_part2(input), 30);
}
