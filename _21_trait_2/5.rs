// Lecture: Traits with generics
trait Investment<T> {
    fn amount(&self) -> T;

    fn double_amount(&mut self);
}

trait Taxable: Investment<f64> { // so each time we're using Taxable, Investment trait will set its T generic as f64.
    const TAX_RATE: f64 = 0.25;
 
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

#[derive(Debug)]
struct Income {
    value: f64
}

impl Investment<f64> for Income {
    fn amount(&self) -> f64 {
        self.value
    }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Income {}

#[derive(Debug)]
struct Bonus {
    value: f64
}

impl Investment<f64> for Bonus {
    fn amount(&self) -> f64 {
        self.value
    }

    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}

impl Taxable for Bonus {
    const TAX_RATE: f64 = 0.50;
}

#[derive(Debug)]
struct QualityTime {
    minutes: u32,
}

impl Investment<u32> for QualityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }

    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}

fn main() {
    let mut income = Income { value: 50000.50 };
    let mut bonus = Bonus { value: 100000.23 };
    let mut rust_programming_time = QualityTime { minutes: 180 };
    income.double_amount();
    bonus.double_amount();
    rust_programming_time.double_amount();

    println!("{rust_programming_time:#?}");
}

// set_amount function was deleted just coz it not used anymore, it was just for setter lesson purpose.