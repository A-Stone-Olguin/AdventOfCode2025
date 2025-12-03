use crate::{traits::day::Day, utils::parse_input_data};

#[derive(Default)]
pub struct Day3;

fn parse_banks(input: &str) -> Vec<&str> {
    parse_input_data::parse_input_to_rows(input)
}

fn get_largest_voltage(bank: &str, number_digits: usize) -> u64 {
    let mut digits: Vec<u64> = vec![0; number_digits];
    // Get each digit
    for (i, c) in bank.char_indices() {
        let digit: u64 = c.to_digit(10).unwrap_or_default().into();

        // Check all remaining digits we can use
        //  See if largest digit first, going to end of string at most
        let remaining_vals = bank.len() - i;
        let start = match remaining_vals >= number_digits {
            true => 0,
            false => number_digits - remaining_vals,
        };
        for j in start..number_digits {
            if digit > digits[j] {
                digits[j] = digit;
                digits[j + 1..].fill(0);
                break;
            }
        }
    }

    digits.iter().enumerate().fold(0, |acc, (i, val)| {
        acc + 10_u64.pow((number_digits - i - 1) as u32) * val
    })
}

fn get_total_voltage(banks: &Vec<&str>, number_digits: usize) -> u64 {
    banks.iter().fold(0, |acc, bank| {
        acc + get_largest_voltage(bank, number_digits)
    })
}

impl Day for Day3 {
    fn part1(&self, input: &str) -> String {
        let banks = parse_banks(input);
        get_total_voltage(&banks, 2).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let banks = parse_banks(input);
        get_total_voltage(&banks, 12).to_string()
    }
}

#[cfg(test)]
mod day3_tests_2025 {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    "#;

    #[test]
    fn test_2025_day3_part1() {
        let day = Day3::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "357");
    }

    #[test]
    fn test_2025_day3_part2() {
        let day = Day3::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "3121910778619");
    }
}
