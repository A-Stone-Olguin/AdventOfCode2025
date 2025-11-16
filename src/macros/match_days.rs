macro_rules! match_days {
    ($day_number:expr, $( $num:expr => $day:ty ),* ) => {
        match $day_number {
            $(
                $num => Box::new(<$day>::default()),
            )*
            _ => panic!("Day is not available: {}", $day_number),
        }
    };
}
