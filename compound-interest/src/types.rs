use rust_decimal::Decimal;
use std::ops::Add;

trait MoneyFormatter {
    fn comma_delimited(&self) -> String;
    fn round_dp(&self, dp: usize) -> String;
    fn currency_symbol(&self, symbol: char) -> String;
}

impl MoneyFormatter for String {
    fn comma_delimited(&self) -> String {
        if let Some(decimal_index) = self.find('.') {
            let (integer_part, fractional_part) = self.split_at(decimal_index);

            let integer_comma = integer_part
                .chars()
                .rev()
                .collect::<String>()
                .chars()
                .collect::<Vec<char>>()
                .chunks(3)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<String>>()
                .join(",")
                .chars()
                .rev()
                .collect::<String>();

            format!("{}{}", integer_comma, fractional_part)
        } else {
            // If there's no decimal point, just return the original string
            self.to_string()
        }
    }

    fn round_dp(&self, dp: usize) -> String {
        if let Some(decimal_index) = self.find('.') {
            let end_index = decimal_index + dp + 1; // Add 1 to include the decimal point
            match self.get(..=end_index) {
                Some(truncated) => truncated.to_string(),
                None => self.to_string(), // Return the original string if end_index is out of bounds
            }
        } else {
            self.to_string() // Return the original string if there's no decimal point
        }
    }

    fn currency_symbol(&self, symbol: char) -> String {
        format!("{}{}", symbol, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(pub Decimal);

impl Money {
    pub fn new(cents: i64) -> Money {
        Money(Decimal::new(cents as i64, 2))
    }
    pub fn with_precision(value: i64, precision: u32) -> Money {
        Money(Decimal::new(value, precision))
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Add commas to the number, and use round_dp
        write!(f, "{}", self.0.round_dp(2))
    }
}

impl From<Decimal> for Money {
    fn from(value: Decimal) -> Self {
        Money(value)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Money(self.0 + other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_display() {
        let money = Money::new(50_000_00)
            .to_string()
            .comma_delimited()
            .currency_symbol('$');
        assert_eq!(format!("{}", money), "$50,000.00");
    }

    #[test]
    fn test_money_add() {
        let money1 = Money::new(50_000_00);
        let money2 = Money::new(50_00);
        let result = money1 + money2;
        assert_eq!(result, Money::new(50_050_00));
    }
}
