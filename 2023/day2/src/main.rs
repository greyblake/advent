fn main() {
    let input = include_str!("input.txt");
    let games = parser::parse_games(input);
    let sum_of_ids = games
        .iter()
        .filter(|game| is_possible_game(game))
        .map(|game| game.id)
        .sum::<i32>();
    println!("Sum: {sum_of_ids}");
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    grabs: Vec<Grab>,
}

// Represents a single grab within a game with number red, green, and blue cubes
#[derive(Default, Debug, PartialEq)]
struct Grab {
    blue: i32,
    red: i32,
    green: i32,
}

fn is_possible_game(game: &Game) -> bool {
    game.grabs.iter().all(is_possible_grab)
}

fn is_possible_grab(grab: &Grab) -> bool {
    grab.red <= 12 && grab.green <= 13 && grab.blue <= 14
}

mod parser {
    use super::*;

    use nom::{
        bytes::complete::tag,
        character::complete::{alphanumeric1, digit1, space0, space1},
        combinator::{fail, map},
        multi::separated_list1,
        IResult,
    };

    // Parse games separated by a new line
    pub fn parse_games(input: &str) -> Vec<Game> {
        games(input).expect("Failed to parse").1
    }

    fn games(input: &str) -> IResult<&str, Vec<Game>> {
        separated_list1(tag("\n"), game)(input)
    }

    #[derive(Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    // Parses inputs like
    // Game 1: 4 green, 3 blue, 11 red; 7 red, 5 green, 10 blue; 3 green, 8 blue, 8 red; 4 red, 12 blue; 15 red, 3 green, 10 blue
    fn game(input: &str) -> IResult<&str, Game> {
        let (input, _) = space0(input)?;
        let (input, _) = tag("Game")(input)?;
        let (input, _) = space1(input)?;
        let (input, id) = digit1(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space1(input)?;
        let (input, grabs) = separated_list1(semicolon, grab)(input)?;
        Ok((
            input,
            Game {
                id: id.parse().unwrap(),
                grabs,
            },
        ))
    }

    fn comma(input: &str) -> IResult<&str, &str> {
        let (input, _) = space0(input)?;
        let (input, _) = tag(",")(input)?;
        space0(input)
    }

    fn semicolon(input: &str) -> IResult<&str, &str> {
        let (input, _) = space0(input)?;
        let (input, _) = tag(";")(input)?;
        space0(input)
    }

    fn grab(input: &str) -> IResult<&str, Grab> {
        map(separated_list1(comma, number_color), |list| {
            list.into_iter()
                .fold(Grab::default(), |mut grab, (number, color)| {
                    match color {
                        Color::Red => grab.red = number,
                        Color::Green => grab.green = number,
                        Color::Blue => grab.blue = number,
                    }
                    grab
                })
        })(input)
    }

    fn number_color(input: &str) -> IResult<&str, (i32, Color)> {
        let (input, number) = digit1(input)?;
        let (input, _) = space1(input)?;
        let (input, color) = color(input)?;
        Ok((input, (number.parse().unwrap(), color)))
    }

    fn color(input: &str) -> IResult<&str, Color> {
        let (input, color) = alphanumeric1(input)?;
        match color {
            "red" => Ok((input, Color::Red)),
            "green" => Ok((input, Color::Green)),
            "blue" => Ok((input, Color::Blue)),
            _ => fail(input),
        }
    }

    #[test]
    fn should_parse_number_color() {
        assert_eq!(number_color("142 red;").unwrap(), (";", (142, Color::Red)));
        assert_eq!(
            number_color("142 green").unwrap(),
            ("", (142, Color::Green))
        );
        assert_eq!(number_color("1 blue").unwrap(), ("", (1, Color::Blue)));
    }

    #[test]
    fn should_parse_grab() {
        assert_eq!(
            grab("1 green, 9 blue;").unwrap(),
            (
                ";",
                Grab {
                    red: 0,
                    green: 1,
                    blue: 9
                }
            )
        );
    }

    #[test]
    fn should_parse_game() {
        assert_eq!(
            game("Game 15: 6 blue; 4 blue; 1 red, 16 blue, 3 green").unwrap(),
            (
                "",
                Game {
                    id: 15,
                    grabs: vec![
                        Grab {
                            red: 0,
                            green: 0,
                            blue: 6
                        },
                        Grab {
                            red: 0,
                            green: 0,
                            blue: 4
                        },
                        Grab {
                            red: 1,
                            green: 3,
                            blue: 16
                        },
                    ]
                }
            )
        );
    }
}
