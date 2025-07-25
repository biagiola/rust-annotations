// Lecture: Supertraits I (Trait inheritance)
// Investment is the supertrait (parent)
// Taxable is the subtrait (child)
// Any type implementing Taxable gets access to both trait's methods

trait Investment {
    fn amount(&self) -> f64;

    fn set_amount(&mut self, new_amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}

// A struct that implements Taxable must also implement Investment (the supertrait).
// The struct gets access to methods from both traits.
// Also, Taxable requires Investment but not the other way around.
trait Taxable: Investment {
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

impl Taxable for Income {}

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
    const TAX_RATE: f64 = 0.50; // overwrite the default value
}

struct QualityTime {
    minutes: f64,
}

impl Investment for QualityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }

    fn set_amount(&mut self, new_amount: f64) {
        self.minutes = new_amount;
    }
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

    let mut weekend = QualityTime { minutes: 120.0 };
    weekend.double_amount();
    println!("Relaxation time: {:.2} minutes", weekend.amount());
}

// What "MUST implement" means:
// MUST implement = You have to provide the functionality
// CAN use = You have access to all methods from both traits
// CHOOSE to use = You decide which methods to actually call