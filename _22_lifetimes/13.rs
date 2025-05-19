// Lecture: Lifetime elision rules I
// // First Elision Rule: The compiler assigns a lifetime to eachparameter that is a reference.
// fn my_awesome_function<'a, 'b>(value: &'a i32, second: &'b str, third: String) -> &'a i32 { ... }

// Second Elision Rule: if there is one reference parameter and the return value is a reference,
// the borrow checker will infer that thier lifetimes are related.
fn my_awesome_function<'a>(value: &'a i32, doesnotmatter: String) -> &'a i32 {
    &value
}
// for the second example, lifetime it's applied to references, not just normal variables
// also does not matter the order, value can be the second parameter that it will be ok too.

// So we can delete our manually lifetimes annotations, install a rust analyser vs code extension
// and we will see those elision rules and the annotations working behind the scenes.

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities = {
        let cities_reference = &cities;
        select_first_two_elements(cities_reference)
    };
    println!("{two_cities:?}");

    {
        let coffees = [
            String::from("Latte"),
            String::from("Mocha"),
        ];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{two_coffees:?}");
    }
}

// So kind of obvious to infer this because it's the only logical way it could work, right?
// A return value cannot be a reference to an owned value, whether it be an original piece of data or
// a parameter (and avoid dangling reference of course).
// And if there's only one reference in the parameter list, then the two must be related, right?
// That is, that is an obvious thing because it is the only possibility at least, right now in our examples.