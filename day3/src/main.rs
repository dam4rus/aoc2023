use std::iter;

#[derive(Debug)]
struct Rect {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Rect {
    fn contains(&self, point: (u32, u32)) -> bool {
        point.0 >= self.x1 && point.0 <= self.x2 && point.1 >= self.y1 && point.1 <= self.y2
    }

    fn inflated(&self) -> Rect {
        Rect {
            x1: self.x1.saturating_sub(1),
            y1: self.y1.saturating_sub(1),
            x2: self.x2.saturating_add(1),
            y2: self.y2.saturating_add(1),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SchemaPartKind {
    Number(String),
    Symbol(char),
}

#[derive(Debug, PartialEq, Eq)]
struct SchemaPart {
    position: (u32, u32),
    kind: SchemaPartKind,
}

impl SchemaPart {
    fn area(&self) -> Rect {
        let width = match &self.kind {
            SchemaPartKind::Number(s) => s.len() as u32,
            _ => 1,
        };
        Rect {
            x1: self.position.0,
            y1: self.position.1,
            x2: self.position.0.saturating_add(width - 1),
            y2: self.position.1,
        }
    }

    fn surrounding_area(&self) -> Rect {
        self.area().inflated()
    }
}

fn parse_input(input: &str) -> Vec<SchemaPart> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            iter::from_fn({
                let mut iter = line.chars().enumerate().peekable();
                move || {
                    while let Some((x, c)) = iter.next() {
                        if c == '.' {
                            continue;
                        }
                        let schema_part_kind = if c.is_digit(10) {
                            let mut part_number = String::from(c);
                            while let Some((_, c)) = iter.peek() {
                                if c.is_digit(10) {
                                    part_number.push(*c);
                                    iter.next();
                                } else {
                                    break;
                                }
                            }
                            SchemaPartKind::Number(part_number)
                        } else {
                            SchemaPartKind::Symbol(c)
                        };
                        return Some(SchemaPart {
                            position: (x as u32, y as u32),
                            kind: schema_part_kind,
                        });
                    }
                    None
                }
            })
            .collect::<Vec<_>>()
        })
        .collect()
}

fn sum_of_valid_number_parts(schema_parts: &Vec<SchemaPart>) -> u32 {
    let (numbers, symbols): (Vec<_>, Vec<_>) = schema_parts
        .iter()
        .partition(|schema_part| matches!(schema_part.kind, SchemaPartKind::Number(_)));
    let sum: u32 = numbers
        .iter()
        .filter(|number| {
            let surrounding_area = number.surrounding_area();
            // println!("surrounding_area: {:?}={:?}", number, surrounding_area);
            symbols.iter().any(|symbol| {
                // println!("symbol: {:?}", symbol);
                surrounding_area.contains(symbol.position)
            })
        })
        .map(|number| {
            if let SchemaPartKind::Number(n) = &number.kind {
                // println!("{}", n);
                n.parse::<u32>().expect("Should be a number")
            } else {
                unreachable!("SchemaPartKind should be Number");
            }
        })
        .sum();
    sum
}

fn sum_of_gear_ratios(schema_parts: &Vec<SchemaPart>) -> u32 {
    let (gears, numbers) = {
        let mut gears = Vec::new();
        let mut numbers = Vec::new();
        for schema_part in schema_parts {
            match schema_part.kind {
                SchemaPartKind::Number(_) => numbers.push(schema_part),
                SchemaPartKind::Symbol('*') => gears.push(schema_part),
                _ => (),
            }
        }
        (gears, numbers)
    };
    gears
        .iter()
        .filter_map(|gear| {
            let adjacent_numbers: Vec<&str> = numbers
                .iter()
                .filter(|number| number.surrounding_area().contains(gear.position))
                .map(|number| {
                    if let SchemaPartKind::Number(n) = &number.kind {
                        n.as_str()
                    } else {
                        unreachable!("SchemaPartKind should be Number");
                    }
                })
                .collect();
            match adjacent_numbers[..] {
                [first, second] => Some(
                    first.parse::<u32>().expect("Should be a number")
                        * second.parse::<u32>().expect("Should be a number"),
                ),
                _ => None,
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let schema_parts = parse_input(input);
    println!(
        "sum of valid number parts: {}",
        sum_of_valid_number_parts(&schema_parts)
    );
    println!("sum of gears: {}", sum_of_gear_ratios(&schema_parts));
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input, sum_of_gear_ratios, sum_of_valid_number_parts, SchemaPart, SchemaPartKind,
    };

    const TEST_INPUT: &'static str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse_schema_parts() {
        let schema_parts = parse_input(TEST_INPUT);
        assert_eq!(
            schema_parts[0],
            SchemaPart {
                position: (0, 0),
                kind: SchemaPartKind::Number(String::from("467")),
            }
        );
        assert_eq!(
            schema_parts[2],
            SchemaPart {
                position: (3, 1),
                kind: SchemaPartKind::Symbol('*'),
            },
        );
    }

    #[test]
    fn test_part_1() {
        let schema_parts = parse_input(TEST_INPUT);
        let sum = sum_of_valid_number_parts(&schema_parts);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_part_2() {
        let schema_parts = parse_input(TEST_INPUT);
        let sum = sum_of_gear_ratios(&schema_parts);
        assert_eq!(sum, 467835);
    }
}
