// How to keep ownership of the data
// Option 2: Use References (Vec<&String>)

fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let mut pizza_toppings: Vec<&String> = vec![&pepperoni, &mushroom, &sausage];

    // To modify one element from the vector 
    // we need to create the new topping as a variable first
    let olives: String = String::from("Olives");
    
    // Now we can reference the olives variable
    pizza_toppings[1] = &olives;

    // we cannot do directly this way. It creates a reference to that temporary string
    // that is dropped immediately after the line finishes..
    // pizza_toppings[1] = &String::from("Olives");

    println!("{pizza_toppings:#?}");

    // all the variables are still available!
    println!("{pepperoni}");
    println!("{mushroom}");
    println!("{sausage}");
    // println!("{olives}");
}

/*
OPTION 2: USING REFERENCES (Vec<&String>)

This approach avoids duplicating data by storing references (borrows) to the original strings but it's
just for reading purposes.

---

1. HOW IT WORKS: BORROWING, NOT OWNING
- The vector does NOT own any string data. It only holds references that point to the data owned by the original variables (`pepperoni`, `mushroom`, etc.).
- This is very memory-efficient as you are only copying pointers, not the full data on the heap.

---

2. THE LIFETIME CHALLENGE: WHY A DIRECTY ASIGNATION TO A VARIABLE USING `&String::from("Olives")` FAILS?
- Rust must guarantee that no reference ever outlives the data it points to. This prevents "dangling references."
- The line `pizza_toppings[1] = &String::from("Olives");` would create a DANGLING REFERENCE. Here's why:
  1. `String::from("Olives")` creates a TEMPORARY `String` value for asignation to the variable.
  2. `&` takes a reference to this temporary value.
  3. At the end of the line (the semicolon), the temporary `String` is dropped and its memory is freed.
  4. The vector would now hold a reference pointing to invalid memory.
- The Rust compiler correctly forbids this at compile time to ensure memory safety.

---

3. THE SOLUTION: USE A NAMED VARIABLE
- By creating `let olives = String::from("Olives");` first, you create a `String` that is owned by the `olives` variable.
- The `olives` variable lives until the end of the `main` function's scope.
- Therefore, the reference `&olives` is guaranteed to be valid for as long as the vector needs it in this scope.

---

4. WHEN TO USE THIS APPROACH
- When you want to avoid the performance cost of cloning.
- When the vector is primarily for reading/iterating over data that is owned elsewhere.
- When the vector will not outlive the original data sources.
*/