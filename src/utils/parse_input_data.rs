use ndarray::Array2;
use regex::Regex;

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

#[cfg(test)]
pub mod parse_input_data_test {
    use regex::Regex;
    use crate::utils::parse_input_data::{parse_input_to_grid, parse_input_to_rows, matches_per_row};

    const EXAMPLE_INPUT: &str = r#"
    1234567890
    1234567890
    1234567890
    1234567890
    "#;

    #[test]
    fn test_parse_input_to_grid() {
       let array = parse_input_to_grid(EXAMPLE_INPUT);
       assert_eq!(array[(0,0)], '1'); 
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
}