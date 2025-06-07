// Lecture: the unwrap_or_else method
// this method allow us to pass a closure as a fallback
// not just a regular &str or any other type of variables

fn main() {
    let option: Option<&str> = Some("Salami");
    let food: &str = option.unwrap_or_else(|| "Pizza");
    println!("{food}");

    // let option: Option<&str> = None;
    // let food: &str = option.unwrap_or_else(|| "Pizza");
    // println!("{food}");

    let option: Option<&str> = None;
    let pizza_fan: bool = false;
    let closure = || {
        if pizza_fan {
            "Pizza"
        } else {
            "Hot Pockets"
        }
    };
    let food: &str = option.unwrap_or_else(closure);
    println!("{food}");
    

    // Now what is the advantage of the closure? Well, now
    // we have execute multiple lines of logic that also has
    // access to data from the outside scope.
    // for example we can have a variable comming from a db,
    // file or the body of a http request, so we can use that
    // in our closure an make a more dynamic procesdure for
    // our else or fallback.
}
