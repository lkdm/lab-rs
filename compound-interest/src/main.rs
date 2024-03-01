#[macro_use]
mod macros;
use compound_interest::Currency;
use rust_decimal_macros::dec;

fn main() {
    // let mut principle = Currency(dec!(41303.15)) * dec!(0.01);
    // let mut principle = currency!(41303.15) * dec!(0.05);
    // println!("{}", principle);
    let mut p = currency!(51303.15);
    let r = dec!(0.05) / dec!(12) + dec!(1);
    let mut total_interest = currency!(0);
    let monthly_deposit = currency!(4_000);

    for month in 0..17 {
        if month == 2 {
            p -= currency!(20_000);
        }
        if month == 4 {
            p -= currency!(10_000);
        }
        p += monthly_deposit;
        let i = p * r - p;
        total_interest += i;
        p += i; // Add the interest to the principal for compounding
    }

    println!("Total Interest over 2 years: {}", total_interest);
    println!("New principle: {}", p)
}
