use std::{fs, io};

fn help_message(day: u8) -> String {
    format!(
        "Try getting the file from https://adventofcode.com/2025/day/{}/input.\n\
         Note that you will need to be logged in.\n\
         Please put the input in the file named `input/Day{}.txt`\n",
        day, day
    )
}

fn day_input_help_message(day: u8) -> impl FnOnce(io::Error) -> io::Error {
    let help_message = help_message(day);
    move |err| io::Error::other(format!("Error: {err}\n\n{help_message}"))
}

pub fn get_input_data(day: u8) -> io::Result<String> {
    let input_file = format!("{}/input/Day{}.txt", env!("CARGO_MANIFEST_DIR"), day);
    fs::read_to_string(input_file).map_err(day_input_help_message(day))
}
