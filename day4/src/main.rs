use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    Finish, IResult,
};

#[derive(Debug, Clone)]
struct CardPair {
    winning_numbers: HashSet<u32>,
    elf_numbers: HashSet<u32>,
    copies: usize,
}

impl CardPair {
    fn score(&self) -> usize {
        self.winning_numbers
            .intersection(&self.elf_numbers)
            .fold(0, |acc, _| match acc {
                0 => 1,
                n => n * 2,
            })
    }

    fn winning_number_count(&self) -> usize {
        self.winning_numbers.intersection(&self.elf_numbers).count()
    }
}

fn parse_card_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, map_res(digit1, str::parse))(input)
}

fn parse_card(input: &str) -> IResult<&str, CardPair> {
    let input = tuple((tag("Card"), space0, digit1, tag(":"), space1))(input)?.0;
    let (input, winning_numbers) = parse_card_numbers(input)?;
    let input = tuple((space1, tag("|"), space1))(input)?.0;
    let (input, elf_numbers) = parse_card_numbers(input)?;
    Ok((
        input,
        CardPair {
            winning_numbers: winning_numbers.into_iter().collect(),
            elf_numbers: elf_numbers.into_iter().collect(),
            copies: 1,
        },
    ))
}

fn parse_cards(input: &str) -> Result<Vec<CardPair>, String> {
    separated_list1(newline, parse_card)(input)
        .finish()
        .map(|ok| ok.1)
        .map_err(|err| err.to_string())
}

fn part_1(cards: &Vec<CardPair>) -> usize {
    cards.iter().map(CardPair::score).sum()
}

fn part_2(cards: &Vec<CardPair>) -> usize {
    let mut cards = cards.clone();
    let mut scratch_card_count = 0;
    for i in 0..cards.len() {
        let copies = cards[i].copies;
        scratch_card_count += copies;
        let winning_number_count = cards[i].winning_number_count();
        for card in &mut cards[i + 1..i + 1 + winning_number_count] {
            card.copies += copies
        }
    }
    scratch_card_count
}

fn main() {
    let input = include_str!("../input.txt");
    let cards = parse_cards(input).unwrap();
    println!("score: {}", part_1(&cards));
    println!("scratch card count: {}", part_2(&cards));
}

#[cfg(test)]
mod tests {
    use crate::{parse_cards, part_2};

    const TEST_INPUT: &'static str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part_1() {
        let score: usize = parse_cards(TEST_INPUT)
            .unwrap()
            .into_iter()
            .map(|card_pair| card_pair.score())
            .inspect(|score| println!("score: {}", score))
            .sum();

        assert_eq!(score, 13);
    }

    #[test]
    fn test_part_2() {
        let cards = parse_cards(TEST_INPUT).unwrap();
        assert_eq!(part_2(&cards), 30);
    }
}
