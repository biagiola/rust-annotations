// Lecture: Flap map method

fn main() {
    let attendees = [
        "Bod, Mary, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    // let attendees: Vec<&str> = attendees
    //     .iter()
    //     .map(|group| group.split(", "))
    //     .flatten()
    //     .collect();
    // println!("{attendees:?}");

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{attendees:?}");
}