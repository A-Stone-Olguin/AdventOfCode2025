use std::time::SystemTime;

pub fn record_timings<F>(number_iterations: u32, data: &str, timed_function: F) -> Vec<i64>
where
    F: Fn(&str) -> String,
{
  (0..number_iterations).map(|_| {
    let start = SystemTime::now();
    timed_function(data);
    match start.elapsed() {
        Ok(elapsed) => elapsed.as_millis() as i64,
        Err(_) => 0,
    }
  }).collect::<Vec<i64>>()
}
