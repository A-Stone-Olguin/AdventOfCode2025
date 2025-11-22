use crate::traits::day::Day;

pub trait Year {
    fn day_factory(&self, day_number: u8) -> Box<dyn Day>;
    fn get_all_days(&self) -> Vec<Box<dyn Day>>;
}
