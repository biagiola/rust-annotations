// Lecture: Generic Lifetimes
// remainder:
// . A generic is a placeholder for a future type.
// . Generics add flexibility byt no hardcoding an exact type
// . Code can use a variety of types in place of the generic

// Generic types vs concrete lifetimes
// . A concrete lifetime is the region of code that a value exists
// in the program (the time it lives in its memory address)
// . A generic lifetime is more abstract. It is a hypothetical
// lifetime, a non-specific lifetime, a future lifetime that can vary
// . We can annotate generic lifetimes in our code. This enables
// functions that are flexible enough to handle varying lifetimes.

// In order to declare generic lifetimes, we use lifetime annotations.
// . A lifetime annotation is a name or label for a lifetime
// . Lifetime annotations don't change the refernce's lifetime. They
// don't affect the logic in any way.
// . A lifetime annotation is a piece of metadata that we provide to the
// borrow checker so that it can validate that references are valid.

// signature of select fn in the previous lecture
// fn select_first_two_elements(items: &[String]) -> &[String] {}

// the key takeaway here is that we are using 'a as a marker to indicate that
// the item's parameter's lifetime is related to the return value lifetime
fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
    // We are saying for some generic hypothetical lifetime called 'a, the
    // returned reference must live within the lifetime of the referent.
    // that 'items' is a reference to, because we have also marked this with 'a.
    // By specifying 'a as the lifetime, we are declaring that the returned
    // reference, &'a [String], cannot outlive the referent for which 'items'
    // is a reference, which thus prevents dangling references.
    // We are explicitly annotating what Rust inferred in the previous lesson.

    // Take it in mind that, the connection is not really betwwen the return
    // value and 'items'. It's more between the return value and the referent
    // from which 'items' comes from, the original source of data that 'items'
    // is a reference to.
    //
    //
}

fn main() {
    let cities: Vec<String> = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let two_cities: &[String] = select_first_two_elements(&cities);
    println!("{two_cities:?}");

    {
        let coffees: [String; 2] = [
            String::from("Latter"),
            String::from("Mocha"),
        ];
        let two_coffees: &[String] = select_first_two_elements(&coffees);
        println!("{two_coffees:?}")
    }
}