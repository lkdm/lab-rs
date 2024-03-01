use rust_decimal::{Decimal, RoundingStrategy};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Currency(pub Decimal);

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .0
            .round_dp_with_strategy(2, RoundingStrategy::MidpointNearestEven);
        write!(f, "${}", formatted)
    }
}

impl Add<Currency> for Currency {
    type Output = Currency;

    fn add(self, rhs: Currency) -> Currency {
        Currency(self.0 + rhs.0)
    }
}

impl Sub<Currency> for Currency {
    type Output = Currency;

    fn sub(self, rhs: Currency) -> Currency {
        Currency(self.0 - rhs.0)
    }
}

impl Mul<Currency> for Currency {
    type Output = Currency;

    fn mul(self, rhs: Currency) -> Currency {
        Currency(self.0 * rhs.0)
    }
}

impl Div<Currency> for Currency {
    type Output = Currency;

    fn div(self, rhs: Currency) -> Currency {
        Currency(self.0 / rhs.0)
    }
}

impl Mul<Decimal> for Currency {
    type Output = Currency;

    fn mul(self, rhs: Decimal) -> Currency {
        Currency(self.0 * rhs)
    }
}

impl Div<Decimal> for Currency {
    type Output = Currency;

    fn div(self, rhs: Decimal) -> Currency {
        Currency(self.0 / rhs)
    }
}

impl Add<Decimal> for Currency {
    type Output = Currency;

    fn add(self, rhs: Decimal) -> Currency {
        Currency(self.0 + rhs)
    }
}

impl Sub<Decimal> for Currency {
    type Output = Currency;

    fn sub(self, rhs: Decimal) -> Currency {
        Currency(self.0 - rhs)
    }
}

impl AddAssign<Currency> for Currency {
    fn add_assign(&mut self, other: Currency) {
        self.0 += other.0;
    }
}

impl SubAssign<Currency> for Currency {
    fn sub_assign(&mut self, other: Currency) {
        self.0 -= other.0;
    }
}

#[test]
fn test_currency_arithmetic() {
    let currency1 = Currency(Decimal::new(1000, 2)); // $10.00
    let currency2 = Currency(Decimal::new(500, 2)); // $5.00

    // Addition
    assert_eq!(currency1 + currency2, Currency(Decimal::new(1500, 2))); // $15.00

    // Subtraction
    assert_eq!(currency1 - currency2, Currency(Decimal::new(500, 2))); // $5.00

    // Multiplication
    assert_eq!(currency1 * currency2, Currency(Decimal::new(5000, 2))); // $50.00

    // Division
    assert_eq!(currency1 / currency2, Currency(Decimal::new(200, 2))); // 2.0

    // Multiplication with Decimal
    let decimal = Decimal::new(250, 2); // $2.50
    assert_eq!(currency1 * decimal, Currency(Decimal::new(2500, 2))); // $25.00

    // Division with Decimal
    assert_eq!(currency1 / decimal, Currency(Decimal::new(4, 0))); // 4.0
}
