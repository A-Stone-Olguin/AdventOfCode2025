#[derive(Default)]
pub struct TimingResult {
    pub day: u8,
    pub part: u8,
    pub year: u32,
    pub min_time_micro: u64,
    pub median_time_micro: f64,
    pub max_time_micro: u64,
    pub number_iterations: u64,
}
