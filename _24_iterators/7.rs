// Lecture: The iter_mut method
// This method will create an iterato that yields mutable
// references to the collection's elements

fn main() {
    let mut flavors = [
        String::from("Chocolate"),
        String::from("Vanilla"),
        String::from("Strawberry"),
    ];

    // let iterator = flavors.iter_mut();

    // for flavor in iterator {
    // for flavor in flavors.iter_mut() {
    for flavor in &mut flavors {
        flavor.push_str("Ice cream");
    }

    println!("{flavors:?}");

    // some times we need to use *'s, for example... 
    let mut school_grades = [85, 90, 72, 92];

    for grade in &mut school_grades {
        // here we need to tell rust that we dont want to substract
        // 2 to the reference, rather to the value that the reference
        // is pointing to, so we need to provide the dereference operator
        // in front of the grade variable.
        *grade -= 2;
    }
}

// SUMMARIZE

// OWNERSHIP
// for value in collection
// for value in collection.into_iter()

// IMMUTABLE REFERENCES
// for value in &collection
// for value in collection.iter()

// MUTABLE REFERENCES
// for value in &mut collection
// for value in collection.iter_mut()