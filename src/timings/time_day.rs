use crate::{
    timings::record_timings::record_timings,
    traits::{day::Day, timing_repository::TimingRepository},
};

pub fn time_day<E>(
    number_iterations: u32,
    day_number: u8,
    data: &str,
    day: &dyn Day,
    timing_repository: &mut Box<dyn TimingRepository<E>>,
) -> Result<(), E> {
    let timings_part1 = record_timings(number_iterations, data, |input| day.part1(input));
    let timings_part2 = record_timings(number_iterations, data, |input| day.part2(input));

    for (i, vector) in [timings_part1, timings_part2].iter().enumerate() {
        let day_id: i64 = (2 * (day_number - 1) + i as u8) as i64;

        timing_repository.delete_day_timings(day_id)?;
        timing_repository.insert_timings(day_id, vector)?;
    }
    Ok(())
}
