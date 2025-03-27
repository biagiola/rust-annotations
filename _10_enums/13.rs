// The let else construct, introduced with rust 1.6.5
enum Milk {
    Lowfat(i32),
    Whole,
    NonDairy{ kind: String }
}

fn main() {
    // let's suppose my_beverage is dynamic. Enum example
    let my_beverage: Milk = Milk::Lowfat(2);
    let my_beverage: Milk = Milk::Whole;

    let Milk::Lowfat(percent) = my_beverage else {
        // if compared variables are not equal... enter this else block
        println!("You don't have the lowfat milk");
        
        // and make sure to finish here
        return
    };

    // percent is only declare when compared variables are not equals
    // so in that case we don't enter the else scope
    println!("{percent}% is available here");

    // example with structs
    let my_beverage: Milk = Milk::NonDairy {
        kind: String::from("Oat")
    };

    let Milk::NonDairy { kind } = my_beverage else {
        println!("You do not have the nondairy milk");
        return
    };
    println!("{kind} milk is available here");
}