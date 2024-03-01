mod types;
// use rust_decimal::Decimal;
// pub use types::{Money, Rational};
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum CalculatorError {
//     DivisionByZero,
// }

// pub trait FinancialCalculator<T> {
//     fn calculate(&self) -> Result<Box<[T]>, CalculatorError>;
// }

// impl CompoundInterest {
//     pub fn new(
//         principle: Money,
//         rate: Rational,
//         contribution: Money,
//         contribution_frequency: AnnualFrequency,
//         compound_frequency: AnnualFrequency,
//         months: u32,
//     ) -> CompoundInterest {
//         CompoundInterest {
//             principle,
//             rate,
//             contribution,
//             contribution_frequency,
//             compound_frequency,
//             months,
//         }
//     }
// }

// pub struct CompoundInterestResult {
//     month: u32,
//     principle: Money,
//     interest: Money,
//     total_interest: Money,
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub enum AnnualFrequency {
//     Annually = 1,
//     SemiAnnually = 2,
//     Quarterly = 4,
//     Monthly = 12,
//     Weekly = 53,
// }

// pub struct CompoundInterest {
//     principle: Money,
//     rate: Rational,
//     contribution: Money,
//     contribution_frequency: AnnualFrequency,
//     compound_frequency: AnnualFrequency,
//     months: u32,
// }

// impl std::fmt::Display for CompoundInterestResult {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Month: {}\tPrinciple: {}\tInterest: {}\tTotal Interest: {}",
//             self.month,
//             self.principle.round_dp(2),
//             self.interest.round_dp(2),
//             self.total_interest.round_dp(2)
//         )
//     }
// }

// impl FinancialCalculator<CompoundInterestResult> for CompoundInterest {
//     fn calculate(&self) -> Result<Box<[CompoundInterestResult]>, CalculatorError> {
//         // Decimal conversions
//         let compound_frequency = Decimal::new((self.compound_frequency as u8).into(), 0);
//         let contribution_frequency = Decimal::new((self.contribution_frequency as u8).into(), 0);

//         let mut p = self.principle;

//         let r = self.rate / compound_frequency;
//         let mut total_interest = Decimal::new(0, 2);
//         let monthly_deposit = self.contribution * contribution_frequency / Decimal::new(12, 0);
//         let mut result = Vec::with_capacity(self.months as usize);
//         for i in 0..self.months {
//             let interest = p * r;
//             total_interest = total_interest + interest;
//             p = p + interest + monthly_deposit;
//             result.push(CompoundInterestResult {
//                 month: i + 1,
//                 principle: p,
//                 interest,
//                 total_interest,
//             });
//         }
//         Ok(result.into_boxed_slice())
//     }
// }
