// The if let construct
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy { kind: String }
}

fn main() {
    // . Enum example
    // let's suppose my_beverage is dynamic
    let my_beverage: Milk = Milk::Lowfat(2);

    if let Milk::Lowfat(percent) = my_beverage {
        println!("You beverage is {percent}% fat milk")
    }

    // example with struct
    let my_beverage: Milk = Milk::NonDairy {
        kind: String::from("Oat")
    };
    if let Milk::NonDairy{ kind } = my_beverage {
        println!("You beverage is {kind} milk")
    } else {
        println!("Some other milk variant")
    }
}