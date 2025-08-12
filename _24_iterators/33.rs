// Lecture: The zip method

fn main() {
    let first_names = ["Casey", "Robert", "Cargo"];
    let last_names = ["Johnson", "Smith", "Rustman"];
    // let last_names = ["Johnson", "Smith", "Rustman", "Anderson"]; // if this is the case, the last one will be ignored

    for (first_name, last_name) in first_names.iter().zip(last_names) {
        println!("{first_name} {last_name}");
    }

    let complete_names: Vec<String> = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| {
            format!("{first_name} {last_name}")
        })
        .collect();
    println!("{complete_names:?}");
}