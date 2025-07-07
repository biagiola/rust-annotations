fn main() {
    let mut action_hero: String = String::from("Arnold Schwarzenegger");

    let ref1: &String = &action_hero; // Immutable borrow
    // let ref2: &mut String = &mut action_hero; // ❌ ERROR: cannot borrow as mutable because it’s already borrowed as immutable

    println!("{ref1}");
}