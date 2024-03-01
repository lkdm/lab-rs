#[macro_use]
mod macros;
use compound_interest::Currency;
use compound_interest::{AnnualFrequency, CompoundInterest, FinancialCalculator};
use rust_decimal_macros::dec;

fn main() {
    let compound_interest = CompoundInterest::new(
        currency!(50_000),
        dec!(0.053),
        currency!(1_000),
        AnnualFrequency::Weekly,
        AnnualFrequency::Monthly,
        48,
    );
    let result = compound_interest.calculate().unwrap();

    for (i, value) in result.iter().enumerate() {
        println!("{}", value);
    }

    // println!("Total Interest over 2 years: {}", total_interest);
    // println!("New principle: {}", p)
}
