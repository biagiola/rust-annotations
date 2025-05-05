// Lecture: Associate constant in a trait
trait Taxable {
    const TAX_RATE: f64 = 0.25; // only the methods here have access to this constant
    fn tax_bill(&self) -> f64;
}

#[derive(Debug)]
struct Income {
    amount: f64
}

impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Bonus() {
    amount: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5; // this will override the Taxable constant

    fn tax_bill(&self) -> f64 {
        self.amount -> Self::TAX_RATE
    }
}

fn main() {
    let income = Income { amount: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let bonus = Bonuys { amount: 100000.23 };
    println!("Bonus tax owned: ${:.2}", income.tax_bill());
}