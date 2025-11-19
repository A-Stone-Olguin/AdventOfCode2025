use crate::traits::year::Year;
use crate::years::*;

pub fn year_factory(year_value: ValidYear) -> Box<dyn Year> {
    match year_value {
        ValidYear::TwentyFive => Box::new(Year2025 {}),
        ValidYear::TwentyFour => Box::new(Year2024 {}),
    }
}
