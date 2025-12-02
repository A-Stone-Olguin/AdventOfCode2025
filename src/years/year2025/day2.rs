use regex::Regex;

use crate::{traits::day::Day, utils::parse_input_data};

#[derive(Default)]
pub struct Day2;

fn parse_ranges(input: &str) -> Vec<(i64, i64)> {
    let rows = parse_input_data::parse_input_to_rows(input);
    let regex = Regex::new(r"\d+-\d+").unwrap();
    let matches = parse_input_data::matches_per_row(rows, &regex);

    matches
        .iter()
        .flatten()
        .filter_map(|range| {
            let (lo, hi) = range.split_once('-')?;
            let lo = lo.parse::<i64>().ok()?;
            let hi = hi.parse::<i64>().ok()?;
            Some((lo, hi))
        })
        .collect()
}

fn get_invalid_ids(lo: i64, hi: i64) -> i64 {
    let mut sum = 0;
    for i in lo..hi + 1 {
        let i_str = i.to_string();
        let (l, r) = i_str.split_at(i_str.len() / 2);
        if l == r {
            sum += i;
        }
    }
    sum
}

fn get_invalid_ids_multiple_repeats(lo: i64, hi: i64) -> i64 {
    let mut sum = 0;
    for i in lo..hi + 1 {
        let i_str = i.to_string();
        for j in 1..i_str.len() / 2 + 1 {
            let (pattern, _) = i_str.split_at(j);
            if i_str.len() % pattern.len() != 0 {
                continue;
            }

            let repeat = pattern.repeat(i_str.len() / pattern.len());
            if i_str == repeat {
                sum += i;
                break;
            }
        }
    }
    sum
}

fn count_invalid_ids(id_ranges: Vec<(i64, i64)>, multiple_repeats: bool) -> i64 {
    let counter_fn = match multiple_repeats {
        false => get_invalid_ids,
        true => get_invalid_ids_multiple_repeats,
    };
    id_ranges
        .iter()
        .fold(0, |prev, (lo, hi)| prev + counter_fn(*lo, *hi))
}

impl Day for Day2 {
    fn part1(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        count_invalid_ids(ranges, false).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let ranges = parse_ranges(input);
        count_invalid_ids(ranges, true).to_string()
    }
}

#[cfg(test)]
mod day2_2025_tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"
    11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_2025_day2_part1() {
        let day = Day2::default();
        assert_eq!(day.part1(EXAMPLE_INPUT), "1227775554")
    }

    #[test]
    fn test_2025_day2_part2() {
        let day = Day2::default();
        assert_eq!(day.part2(EXAMPLE_INPUT), "4174379265")
    }
}

/**
 * Here is my failure to read the problem lol.
 * I thought I needed to count "how many" invalid ids there were,
 * not what those ids exactly were. Here's my leftover scratchwork.
 */
mod day2_2025_scrapyard {
    // 11       (init = 10^1 + 10^0)  (+= 11) (10^1 + 1)
    // 1010     (init = 10^3 + 10^1) (+= 101) (10^2 + 1)
    // 100100   (init = 10^5 + 10^2) (+= 1001) (10 ^3 +1)

    // 10 - 100 -> 9 invalids (9 * (log_10(10)))
    //  log_10(n).floor == 1
    // 100 - 1000 -> 0 invalids
    //  log_10(n).floor == 2
    // 1000 - 10000 -> 10, 11, 12 ,... , 99 -> 90 invalids -> (9 * 10 = log_10())
    //  1010, 1111, 1212 => +=101
    //  log_10(n).floor == 3

    fn _lesser_invalid_ids(value: &i64) -> i64 {
        let mut count = 0;

        // Get all "guaranteed values"
        let hi_log = f32::log10(*value as f32).floor() as i64;
        let mut n = 1;
        while n < hi_log {
            count += 9 * 10_i64.pow(((n - 1) / 2).try_into().unwrap());
            n += 2;
        }

        // Get all "remaining"
        if hi_log % 2 != 0 {
            let initial_invalid_val = 10_i64.pow((hi_log).try_into().unwrap())
                + 10_i64.pow(((hi_log - 1) / 2).try_into().unwrap());
            let scalar = 10_i64.pow(hi_log.try_into().unwrap()) + 1;
            let mut i = 0;
            while initial_invalid_val + i * scalar <= *value {
                i += 1;
                count += 1;
            }
        }
        count
    }
}
