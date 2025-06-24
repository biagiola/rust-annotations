fn main() {
    let mut car: String = String::from("Red"); // 'car' is a mutable String

    let ref1: &mut String = &mut car;          // `ref1` is a mutable reference to `car`

    ref1.push_str(" and Blue");
    println!("{ref1}");

    println!("{car}");
    // what happens to car? It was altered by the ref1 mutation
    // but it still exists
}