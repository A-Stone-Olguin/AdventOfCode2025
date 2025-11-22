macro_rules! list_days {
    ($($day:ty),* ) => {
        vec![$(Box::new(<$day>::default())), *]
    };
}
