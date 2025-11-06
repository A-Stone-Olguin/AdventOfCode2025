use std::fs;

pub fn get_input_data(path: String) -> String {
    let text = fs::read_to_string(path);
    match text {
        Ok(contents) => contents,
        Err(e) => format!("{}", e),
    }
}