// Mutable Reference Pattern
fn add_last_name(hero_name: &mut String) { //Borrows mutably
    hero_name.push_str(" Schwarzenegger")  // Modifies in place
}

fn main() {
    let mut action_hero: String = String::from("Arnold");
    add_last_name(&mut action_hero);
    println!("{0}", action_hero);
}
