fn main() {
    let mut car: String = String::from("Red"); // 'car' is a mutable String

    let ref1: &mut String = &mut car;          // `ref1` is a mutable reference to `car`

    ref1.push_str(" and Blue");
    println!("{ref1}");

    println!("{car}");
    // what happens to car? It was altered by the ref1 mutation
    // but it still exists
}

// What ref1 Can and Cannot Change:
// ref1 CAN change:
// - The contents of the String (the actual text data)
// - The String's internal fields (length, capacity, heap pointer)
// - Potentially the heap address the String points to (if reallocation happens)
// ref1 CANNOT change:
// - The stack address where car is stored
// The identity of the car variable itself