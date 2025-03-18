fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day.");
}

fn main() {
    let action_hero: String = String::from("Arnold Schwarzenegger");
    do_hero_stuff(&action_hero);

    let another_action_hero: &str = "Sylvester Stallone";
    do_hero_stuff(another_action_hero);
    // the fn will not work if there's a mismatch between
    // ref str and ref String (&str and &String).
    // ref str it is more versatail and rust is complete flexible enough
    // to treat String as &str with the deref coercion. Coz...
    // a ref String (&String) can represented as a slice of the whole with
    // ref str (&str)
    // &String -> &str - is okay, a slice could be the whole string
    // &str -> &String - a slice could not always be the whole &String
}