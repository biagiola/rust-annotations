// Lecture: is_sorted and sort method with structs

#[derive(Debug)]
struct GasStation {
    snack_count: u32,
    manager: String,
    empoyee_count: u32,
}

fn main() {
    let mobil = GasStation {
        snack_count: 100,
        manager: String::from("Meg mobil"),
        empoyee_count: 3,
    };

    let exxon = GasStation {
        snack_count: 130,
        manager: String::from("Eric Exxon"),
        empoyee_count: 4,
    };

    let shell = GasStation {
        snack_count: 50,
        manager: String::from("Shane Shell"),
        empoyee_count: 2,
    };

    let mut stops = [mobil, exxon, shell];
    // sort_by_key() instead of just sort()
    stops.sort_by_key(|station| { station.snack_count});

    // ascending by snack_count
    println!("{stops:?}");

    // if we want to print in descending by snack_count, we can use the negative of that,
    // the only thing is that field is a u32 we cast it first
    stops.sort_by_key(|station| -(station.empoyee_count as i32));
    println!("{stops:?}");
}