// rust 1.5.6 introduced the Hash::from() method
use std::collections::HashMap;

fn main() {
    let data = [
        ("Body", 7),
        ("Grant", 4),
        ("Ben", 6)
    ];

    let mut years_at_company = HashMap::from(data);
    println!("{:?}", years_at_company);

    // it removes and give us its removed value
    let ben: Option<i32> = years_at_company.remove("Ben");
    println!("{:?}", ben); // Some(6)
    println!("{:?}", ben.unwrap()); // 6

    // removed values is gone
    println!("{:?}", years_at_company);

    let ben: Option<i32> = years_at_company.remove("Ben");
    println!("{:?}", ben); // None
}
