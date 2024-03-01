use crate::Currency;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CalculatorError {
    DivisionByZero,
}

pub trait FinancialCalculator<T> {
    fn calculate(&self) -> Result<Box<[T]>, CalculatorError>;
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnnualFrequency {
    Annually = 1,
    SemiAnnually = 2,
    Quarterly = 4,
    Monthly = 12,
    Weekly = 53,
}

pub struct CompoundInterest {
    principle: Currency,
    rate: Decimal,
    contribution: Currency,
    contribution_frquency: AnnualFrequency,
    compound_frequency: AnnualFrequency,
    months: u16,
}

impl CompoundInterest {
    pub fn new(
        principle: Currency,
        rate: Decimal,
        contribution: Currency,
        contribution_frquency: AnnualFrequency,
        compound_frequency: AnnualFrequency,
        months: u16,
    ) -> CompoundInterest {
        CompoundInterest {
            principle,
            rate,
            contribution,
            contribution_frquency,
            compound_frequency,
            months,
        }
    }
}

pub struct CompoundInterestResult {
    month: u16,
    principle: Currency,
    interest: Currency,
    total_interest: Currency,
}

impl std::fmt::Display for CompoundInterestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Month: {}\tPrinciple: {}\tInterest: {}\tTotal Interest: {}",
            self.month, self.principle, self.interest, self.total_interest
        )
    }
}

impl FinancialCalculator<CompoundInterestResult> for CompoundInterest {
    fn calculate(&self) -> Result<Box<[CompoundInterestResult]>, CalculatorError> {
        let mut p = self.principle;
        let r = self.rate / Decimal::from(self.compound_frequency as u8);
        let mut total_interest = Currency(Decimal::from(0));
        let monthly_deposit =
            self.contribution * Decimal::from(self.contribution_frquency as u8) / Decimal::from(12);
        let mut result = Vec::with_capacity(self.months as usize);

        for i in 0..self.months {
            let interest = p * r;
            total_interest = total_interest + interest;
            p = p + interest + monthly_deposit;
            result.push(CompoundInterestResult {
                month: i,
                principle: p,
                interest,
                total_interest,
            });
        }
        Ok(result.into_boxed_slice())
    }
}
