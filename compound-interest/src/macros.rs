macro_rules! currency {
    ($val:expr) => {
        Currency(dec!($val))
    };
}
