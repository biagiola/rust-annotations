fn add_last_name(mut hero_name: String) -> String {
    hero_name.push_str(" Schwarzenegger");
    hero_name
}
fn main() {
    let action_hero: String = String::from("Arnold");
    let new_name: String = add_last_name(action_hero);
    println!("{}", new_name);
}