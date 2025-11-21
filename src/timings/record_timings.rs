use crate::timings::timing_result::TimingResult;
use std::fmt::Write;
use std::fs;
use std::time::SystemTime;

pub fn record_timings<F>(number_iterations: u32, data: &str, timed_function: F) -> Vec<i64>
where
    F: Fn(&str) -> String,
{
    let mut timings: Vec<i64> = vec![];
    for _ in 0..number_iterations {
        let start = SystemTime::now();
        timed_function(data);
        let duration = match start.elapsed() {
            Ok(elapsed) => i64::try_from(elapsed.as_micros()).unwrap_or(i64::MAX),
            Err(_) => 0,
        };
        timings.push(duration);
    }
    timings
}

fn convert_micros_to_millis<T>(microsecond: T) -> f64 
 where
    T: Into<f64>
{
    (microsecond.into()) / 1000.0
}


fn construct_table_string(timings: Vec<TimingResult>) -> Result<String, std::fmt::Error> {
    let mut md_table = String::new();

    writeln!(
        md_table,
        "| Day | Part | Min Time (ms) | Median Time (ms) | Max Time (ms) | Number Iterations |"
    )?;
    writeln!(
        md_table,
        "| --- | ---- | ------------- | ---------------- | ------------- | ----------------- |"
    )?;

    for timing in timings {
        writeln!(
            md_table,
            "| {} | {} | {} | {} | {} | {} |",
            timing.day,
            timing.part,
            convert_micros_to_millis(timing.min_time_micro as f64),
            convert_micros_to_millis(timing.median_time_micro),
            convert_micros_to_millis(timing.max_time_micro as f64),
            timing.number_iterations,
        )?;
    }
    Ok(md_table)
}

fn write_table_to_readme(markdown_table: &str) -> Result<(), std::io::Error> {
    let readme_path = format!("{}/README.md", env!("CARGO_MANIFEST_DIR"));
    let content = fs::read_to_string(&readme_path)?;

    let start_marker = "<!-- START_TIMINGS_TABLE -->";
    let end_marker = "<!-- END_TIMINGS_TABLE -->";

    let start_idx =
        content.find(start_marker).expect("Start marker not found") + start_marker.len();
    let end_idx = content.find(end_marker).expect("End marker not found");

    let new_content = format!(
        "{}\n{}\n{}",
        &content[..start_idx],
        markdown_table.trim(),
        &content[end_idx..]
    );

    fs::write(readme_path, new_content)
}

fn construct_readme_table_for_years(
    mut timings: Vec<TimingResult>,
) -> Result<String, std::io::Error> {
    match timings.len() {
        0 => Ok(String::new()),
        _ => {
            let first_year = timings.first().unwrap_or(&TimingResult::default()).year;
            let partition_point = timings.partition_point(|x| x.year == first_year);
            let rest = timings.split_off(partition_point);
            Ok(format!(
                "### {first_year}\n{}\n{}",
                construct_table_string(timings).map_err(std::io::Error::other)?,
                construct_readme_table_for_years(rest)?
            ))
        }
    }
}

pub fn write_timings_to_readme(timings: Vec<TimingResult>) -> Result<(), std::io::Error> {
    let markdown_table = construct_readme_table_for_years(timings)?;
    write_table_to_readme(&markdown_table)
}
