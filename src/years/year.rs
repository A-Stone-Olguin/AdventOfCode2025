use strum_macros::EnumIter;

#[derive(clap::ValueEnum, Clone, EnumIter)]
pub enum ValidYear {
    TwentyFive,
    TwentyFour,
}

pub fn valid_days(valid_year: ValidYear) -> u8 {
    match valid_year {
        ValidYear::TwentyFive => 12,
        ValidYear::TwentyFour => 25,
    }
}

impl From<u32> for ValidYear {
    fn from(value: u32) -> Self {
        match value {
            2025 => Self::TwentyFive,
            2024 => Self::TwentyFour,
            _ => Self::TwentyFive,
        }
    }
}

impl From<ValidYear> for u32 {
    fn from(value: ValidYear) -> Self {
        match value {
            ValidYear::TwentyFive => 2025,
            ValidYear::TwentyFour => 2024,
        }
    }
}
