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
    fn surrounding_area(&self) -> Rect {
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
        .inflated()
    }
}

fn parse_input(input: &str) -> Vec<SchemaPart> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut schema_parts = Vec::new();
            let mut iter = line.chars().enumerate().peekable();
            while let Some((x, c)) = iter.next() {
                if c == '.' {
                    continue;
                }
                if !c.is_digit(10) {
                    schema_parts.push(SchemaPart {
                        position: (x as u32, y as u32),
                        kind: SchemaPartKind::Symbol(c),
                    })
                } else {
                    let mut part_number = String::from(c);
                    while let Some((_, c)) = iter.peek() {
                        if c.is_digit(10) {
                            part_number.push(*c);
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    schema_parts.push(SchemaPart {
                        position: (x as u32, y as u32),
                        kind: SchemaPartKind::Number(part_number),
                    });
                }
            }
            schema_parts
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
                n.parse::<u32>().expect("should be number")
            } else {
                panic!("should be number");
            }
        })
        .sum();
    sum
}

fn main() {
    let input = include_str!("../input.txt");
    let schema_parts = parse_input(input);
    println!(
        "sum of valid number parts: {}",
        sum_of_valid_number_parts(&schema_parts)
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, sum_of_valid_number_parts, SchemaPart, SchemaPartKind};

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
}
