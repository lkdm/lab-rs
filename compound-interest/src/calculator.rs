enum CalculatorError {
    DivisionByZero,
}

trait FinancialCalculator {
    fn calculate(&self) -> Result<Currency, CalculatorError>;
    fn calculate_series<T>(&self) -> Result<Box<[T]>, CalculatorError>;
}

struct CompoundInterest {
    principle: Currency,
    rate: Decimal,
    frequency: AnnualCompoundFrequency,
    time: u32,
}
