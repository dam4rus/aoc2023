fn calibration_value(input: &str, include_words: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line
                .char_indices()
                .filter_map(|(i, c)| {
                    c.to_digit(10).or_else(|| {
                        if include_words {
                            match &line[i..] {
                                s if s.starts_with("one") => Some(1),
                                s if s.starts_with("two") => Some(2),
                                s if s.starts_with("three") => Some(3),
                                s if s.starts_with("four") => Some(4),
                                s if s.starts_with("five") => Some(5),
                                s if s.starts_with("six") => Some(6),
                                s if s.starts_with("seven") => Some(7),
                                s if s.starts_with("eight") => Some(8),
                                s if s.starts_with("nine") => Some(9),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    })
                })
                .collect();
            (digits.first().expect("digits should not be empty") * 10)
                + digits.last().expect("digits should not be empty")
        })
        .sum()
}

fn solve_part_1(input: &str) -> u32 {
    calibration_value(input, false)
}

fn solve_part_2(input: &str) -> u32 {
    calibration_value(input, true)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Sum of calibration values: {}", solve_part_1(input));
    println!(
        "Sum of calibration values with digits as words: {}",
        solve_part_2(input)
    );
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_PART_1: &'static str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT_PART_2: &'static str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn day_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT_PART_1), 142);
    }

    #[test]
    fn day_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT_PART_2), 281);
    }
}
