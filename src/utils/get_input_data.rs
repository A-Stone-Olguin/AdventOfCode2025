use std::fs;

pub fn get_input_data(path: String) -> String {
    let text = fs::read_to_string(path);
    match text {
        Ok(contents) => return contents,
        Err(e) => return format!("{}", e),
    }
}