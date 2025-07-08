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
    // "&str is more versatile because Rust automatically converts &String to &str
    // through deref coercion, making functions more flexible."
    // &String -> &str - is okay, a slice could be the whole string
    // &str -> &String - a slice could not always be the whole &String
}