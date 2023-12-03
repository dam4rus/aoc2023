use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1, u32},
    combinator::opt,
    multi::{many1, separated_list0, separated_list1},
    sequence::tuple,
    IResult,
};
use std::str::FromStr;
use strum::{AsRefStr, EnumString};

#[derive(AsRefStr, EnumString, Debug)]
#[strum(serialize_all = "snake_case")]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    revealed_cubes_set: Vec<Cubes>,
}

fn parse_revealed_cube(input: &str) -> IResult<&str, (u32, CubeColor)> {
    // println!("parsing revealed cube: {}", input);
    let (input, n) = u32(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((
        tag(CubeColor::Red.as_ref()),
        tag(CubeColor::Green.as_ref()),
        tag(CubeColor::Blue.as_ref()),
    ))(input)?;
    let cube_color = CubeColor::from_str(color).expect("Could not parse color");
    // println!("cube color: {}", cube_color.as_ref());
    Ok((input, (n, cube_color)))
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    // println!("parsing cubes: {}", input);
    let (input, cubes) = separated_list1(tuple((tag(","), space0)), parse_revealed_cube)(input)?;
    let (input, _) = opt(tuple((tag(";"), space0)))(input)?;
    // println!("cubes {:?}", cubes);
    // println!("input {}", input);
    let cubes = cubes.into_iter().fold(Cubes::default(), |mut acc, item| {
        match item.1 {
            CubeColor::Red => acc.red = item.0,
            CubeColor::Green => acc.green = item.0,
            CubeColor::Blue => acc.blue = item.0,
        }
        acc
    });
    Ok((input, cubes))
}

fn parse_cubes_set(input: &str) -> IResult<&str, Vec<Cubes>> {
    // println!("parsing cubes set: {}", input);
    many1(parse_cubes)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // println!("parsing game: {}", input);
    let (input, _) = tag("Game")(input)?;
    let (input, _) = space1(input)?;
    // println!("parsing game id: {}", input);
    let (input, game_id) = u32(input)?;
    let (input, _) = tuple((tag(":"), space0))(input)?;
    let (input, cubes_set) = parse_cubes_set(input)?;
    Ok((
        input,
        Game {
            id: game_id,
            revealed_cubes_set: cubes_set,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(line_ending, parse_game)(input)
}

fn calculate_sum_of_id_of_possible_games(games: &Vec<Game>) -> u32 {
    let sum_of_ids: u32 = games
        .iter()
        .filter_map(|game| {
            game.revealed_cubes_set
                .iter()
                .all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
                .then_some(game.id)
        })
        .sum();
    sum_of_ids
}

fn calculate_sum_of_power(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let min_cubes = game
                .revealed_cubes_set
                .iter()
                .copied()
                .reduce(|acc, e| Cubes {
                    red: acc.red.max(e.red),
                    green: acc.green.max(e.green),
                    blue: acc.blue.max(e.blue),
                })
                .expect("revealed cubes should not be empty");
            min_cubes.red * min_cubes.green * min_cubes.blue
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let games = parse_games(input).unwrap().1;
    let sum_of_ids = calculate_sum_of_id_of_possible_games(&games);
    println!("sum of possible game ids: {}", sum_of_ids);
    println!("sum of power: {}", calculate_sum_of_power(&games));
}

#[cfg(test)]
mod tests {
    use crate::{calculate_sum_of_power, parse_games, Cubes, Game};

    const TEST_INPUT: &'static str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parser() {
        let (_, games) = parse_games(TEST_INPUT).expect("dinnyefa");
        assert_eq!(
            games,
            vec![
                Game {
                    id: 1,
                    revealed_cubes_set: vec![
                        Cubes {
                            red: 4,
                            green: 0,
                            blue: 3
                        },
                        Cubes {
                            red: 1,
                            green: 2,
                            blue: 6
                        },
                        Cubes {
                            red: 0,
                            green: 2,
                            blue: 0
                        }
                    ],
                },
                Game {
                    id: 2,
                    revealed_cubes_set: vec![
                        Cubes {
                            red: 0,
                            green: 2,
                            blue: 1
                        },
                        Cubes {
                            red: 1,
                            green: 3,
                            blue: 4
                        },
                        Cubes {
                            red: 0,
                            green: 1,
                            blue: 1
                        },
                    ],
                },
                Game {
                    id: 3,
                    revealed_cubes_set: vec![
                        Cubes {
                            red: 20,
                            green: 8,
                            blue: 6
                        },
                        Cubes {
                            red: 4,
                            green: 13,
                            blue: 5
                        },
                        Cubes {
                            red: 1,
                            green: 5,
                            blue: 0
                        },
                    ],
                },
                Game {
                    id: 4,
                    revealed_cubes_set: vec![
                        Cubes {
                            red: 3,
                            green: 1,
                            blue: 6
                        },
                        Cubes {
                            red: 6,
                            green: 3,
                            blue: 0
                        },
                        Cubes {
                            red: 14,
                            green: 3,
                            blue: 15
                        },
                    ],
                },
                Game {
                    id: 5,
                    revealed_cubes_set: vec![
                        Cubes {
                            red: 6,
                            green: 3,
                            blue: 1
                        },
                        Cubes {
                            red: 1,
                            green: 2,
                            blue: 2
                        },
                    ],
                }
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let games = parse_games(TEST_INPUT).unwrap().1;
        let sum_of_ids: u32 = games
            .iter()
            .filter_map(|game| {
                game.revealed_cubes_set
                    .iter()
                    .all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
                    .then_some(game.id)
            })
            .sum();
        assert_eq!(sum_of_ids, 8);
    }

    #[test]
    fn test_part_2() {
        let games = parse_games(TEST_INPUT).unwrap().1;
        let sum_of_power = calculate_sum_of_power(&games);
        assert_eq!(sum_of_power, 2286);
    }
}
