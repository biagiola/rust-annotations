// Lecture: is_sorted and sort method with structs

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    employee_count: u32,
}

fn main() {
    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Meg mobil"),
        employee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        employee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        employee_count: 2,
    };

    let mut stops = [mobil, exxon, shell];
    // sort_by_key() instead of just sort()
    // ascending by snack_count
    stops.sort_by_key(|station| { station.snack_count});
    println!("Snack count . ASC ");
    println!("{stops:#?}");

    // if we want to print in descending by snack_count, we can use the negative of that,
    // the only thing is that field is a u32 we cast it first
    stops.sort_by_key(|station| -(station.employee_count as i32));
    println!("\nEmployee count . DESC ");
    println!("{stops:#?}");
}