// Lecture: The cloned method

fn main() {
    // The copied method converts an iterator
    // from storing &T elements to T elements.
    // It makes a copy of each T element.
    // The T data type must implement the Copy trait.

    // The cloned method similarly converts and
    // iterates from storing &T elements to T elements.
    // It makes a clone of each T element.
    // The T data type must implement the Clone trait.
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    // iterator has &String --> String
    let more_teas: Vec<String> = teas
        .iter()
        .cloned()
        .collect();
    println!("{more_teas:?}");
    println!("{teas:?}");

    // it's important to filter before cloned for optimazations
    let more_teas: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Hot"))
        .cloned()
        .collect();
    println!("{more_teas:?}");

}