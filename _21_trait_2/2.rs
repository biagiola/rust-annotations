// Lecture: Getters on a traits
trait Taxable {
    const TAX_RATE: f64 = 0.25;

    // traits cannot guarantee fiels but behaivours, so we force to declare an amount
    // getter for our trait and we called when we need it.
    fn amount(&self) -> f64;
 
    // we set an default behaviour that can be override it.
    // so now both struct can use it here.
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    amount: f64
}

impl Taxable for Income {
    fn amount(&self) -> f64 { // now this method it's mandate it by the trait
        self.amount
    }
}

#[derive(Debug)]
struct Bonus {
    value: f64
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;

    fn amount(&self) -> f64 { // now this method it's mandate it by the trait
        self.value
    }
}

fn main() {
    let income = Income { amount: 50000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let bonus = Bonus { value: 100000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
}