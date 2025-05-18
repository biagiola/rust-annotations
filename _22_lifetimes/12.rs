// Lecture: Lifetimes and referents
// the returned reference is not coupled directly to the items parameter
// but with the reference in the outside call.

// the returned &[String] isn’t tied to the local parameter binding items,
// or to the inner cities_reference), but to the underlying data you passed
// in this case, the cities vector itself.
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
        // notice that cities_reference is drop when the scope ends
        // kind like where a function ends from the previous examples.
        // but, as we said the returned reference is coupled with
        // its referant, in this case two_cities and not with the
        // parameter cities_reference that we pass it.
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