// Lecture: Supertraits I (Trait inheritance)
trait Investment { // supertrait
    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}

// a data struct that implements the Taxable traits must to implement
// all the functionalities from the super and sub trait
trait Taxable: Investment { // subtrait
    const TAX_RATE: f64 = 0.25;
 
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    value: f64
}

impl Investment for Income {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Income {} // implements its default fns and values

#[derive(Debug)]
struct Bonus {
    value: f64
}

impl Investment for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.value = new_amount;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50; // override the default value
}

fn main() {
    let mut income = Income { value: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonus { value: 100000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
}