use ndarray::Array2;
use regex::Regex;

pub type CaptureTuple = Vec<String>;

pub fn parse_input_to_grid(input_data: &str) -> Array2<char> {
    let lines: Vec<&str> = input_data.trim().lines().map(|line| line.trim()).collect();
    let num_rows = lines.len();
    let num_cols = lines.first().map(|l| l.chars().count()).unwrap_or(0);

    Array2::from_shape_fn((num_rows, num_cols), |(i, j)| {
        lines[i].chars().nth(j).unwrap_or(' ')
    })
}

pub fn parse_input_to_rows(input_data: &str) -> Vec<&str> {
    input_data.trim().lines().map(|line| line.trim()).collect()
}

pub fn matches_per_row(rows: Vec<&str>, regex: &Regex) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| {
            regex
                .find_iter(row)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn captures_per_row(rows: Vec<&str>, regex: &Regex) -> Vec<Vec<CaptureTuple>> {
    rows.iter()
        .map(|row| {
            regex
                .captures_iter(row)
                .filter_map(|captures| {
                    captures
                        .iter()
                        .skip(1)
                        .map(|group| group.map(|m| m.as_str().to_string()))
                        .collect::<Option<Vec<_>>>()
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
pub mod parse_input_data_test {
    use crate::utils::parse_input_data::{
        captures_per_row, matches_per_row, parse_input_to_grid, parse_input_to_rows,
    };
    use regex::Regex;

    const EXAMPLE_INPUT: &str = r#"
    1234567890
    1234567890
    1234567890
    1234567890
    "#;

    #[test]
    fn test_parse_input_to_grid() {
        let array = parse_input_to_grid(EXAMPLE_INPUT);
        assert_eq!(array[(0, 0)], '1');
        assert_eq!(array.shape(), &[4, 10]);
    }

    #[test]
    fn test_parse_input_to_rows() {
        let rows = parse_input_to_rows(EXAMPLE_INPUT);
        assert_eq!(rows[0], "1234567890");
        assert_eq!(rows.len(), 4);
    }

    #[test]
    fn test_matches_per_row() {
        let regex = Regex::new(r"\d{2}").unwrap(); // match any 2 digits
        let rows = parse_input_to_rows(EXAMPLE_INPUT);
        let results = matches_per_row(rows, &regex);

        // Each row should have 5 matches: "12", "34", "56", "78", "90"
        assert_eq!(results.len(), 4);
        for row_matches in results {
            assert_eq!(row_matches, vec!["12", "34", "56", "78", "90"]);
        }
    }

    #[test]
    fn test_captures_per_row() {
        let regex = Regex::new(r"(\d)\s+(\d)").unwrap();
        let rows = vec!["1   2", "3   4", "5   6"];
        let results = captures_per_row(rows, &regex);

        assert_eq!(
            results,
            vec![
                vec![vec!["1".to_string(), "2".to_string()]],
                vec![vec!["3".to_string(), "4".to_string()]],
                vec![vec!["5".to_string(), "6".to_string()]]
            ]
        );
    }
}
