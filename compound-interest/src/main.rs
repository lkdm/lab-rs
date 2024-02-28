struct Money(isize);

impl std::ops::Mul<isize> for Money {
    type Output = Money;

    fn mul(self, rhs: isize) -> Money {
        Money(self.0 * rhs)
    }
}

impl std::ops::Mul<Money> for Money {
    type Output = Money;

    fn mul(self, rhs: Money) -> Money {
        // Create a new Money object with the result of the multiplication
        Money(self.0 * rhs.0 / 100)
    }
}

impl std::ops::Mul<f64> for Money {
    type Output = Money;

    fn mul(self, rhs: f64) -> Money {
        Money((self.0 as f64 * rhs) as isize)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs_value = self.0.abs();
        write!(f, "{}{}.{}", sign, abs_value / 100, abs_value % 100)
    }
}

fn main() {
    let total = Money(100) * Money(200) * -1.053;
    println!("total: {}", total);
}
