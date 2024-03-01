// use compound_interest::{AnnualFrequency, CompoundInterest, FinancialCalculator, Money, Rational};
// use rust_decimal::Decimal;

// fn main() {
//     let compound_interest = CompoundInterest::new(
//         Money::from(Decimal::new(50_000_00, 2)),
//         Rational::from(Decimal::new(0_053, 3)),
//         Money::from(1_000),
//         AnnualFrequency::Weekly,
//         AnnualFrequency::Monthly,
//         24,
//     );
//     let result = compound_interest.calculate().unwrap();

//     for (_, value) in result.iter().enumerate() {
//         println!("{}", value);
//     }
// }
