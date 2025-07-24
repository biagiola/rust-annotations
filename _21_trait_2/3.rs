// Lecture: Setters on a traits
trait Taxable {
    const TAX_RATE: f64 = 0.25;

    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }

    fn tax_bill(&self) -> f64 {
        self.amount() *Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64
}

impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.amount = new_amount;
    }
}

#[derive(Debug)]
struct Bonus {
    amount: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.5; // this will override the constant

    fn amount() -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

fn main() {
    let mut income = Income { amount: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 100000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
}

// We cannot avoid repeating the setter implementation on each struct, Bonus and Income.

// In each trait implementation of Taxable, in Bonus and Income, we need to create the set_amount method.
// We cannot define the body of the set_amount in the Taxable trait and try to call self.amount field to
// reach its value because it is not defined there right actually, we just have a signature amount().
// trait Taxable {
//     fn set_amount(&mut self, new_amount: f64) {
//         self.amount = new_amount; // Error: no field `amount` on type `Self`
//     }
// }

// The amount field exists when we define the Bonus or Income instance

