// Lecture: Associate constant in a trait
// You cannot access trait constants through instances. This is by design in Rust.
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
struct Bonus {
    amount: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5; // this will override the Taxable constant

    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}

fn main() {
    let income = Income { amount: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    // let tax_rate = income.TAX_RATE; // there is no way to access the constante
    // Trait constants belong to the type (Taxable), not the instance (income in this case).
    // They're associated with the type itself, similar to static members in other languages.

    let bonus = Bonus { amount: 100000.23 };
    println!("Bonus tax owned: ${:.2}", income.tax_bill());

    // We cannot access the cosntant through the instance of the trait, but there are other ways
    // Method 1: Through the implementing type
    let tax_rate = Income::TAX_RATE;

    // Method 2: Through the trait itself  
    let tax_rate = <Income as Taxable>::TAX_RATE;

    // Method 3: If trait is in scope, directly
    let tax_rate = Taxable::TAX_RATE; // (but this gets the default, not type-specific)
    
}