use crate::db::timing_result::TimingResult;

pub trait TimingRepository<E> {
    fn insert_timings(&mut self, day_id: u8, timings_ms: &[i64]) -> Result<(), E>;
    fn delete_day_timings(&mut self, day_id: u8) -> Result<usize, E>;
    fn get_timings(&mut self) -> Result<Vec<TimingResult>, E>;
}