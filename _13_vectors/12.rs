// How to keep ownership of the data
// Option 4: Mix - Clone Only When Adding to Vector

fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let mut pizza_toppings: Vec<String> = Vec::new();
    pizza_toppings.push(pepperoni.clone());
    pizza_toppings.push(mushroom.clone());
    pizza_toppings.push(sausage.clone());

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    // Originals still available
    println!("I still have: {}, {}, {}", pepperoni, mushroom, sausage);
}

/*
CLONING WITH `push()` - Functionally the same as Option 1

This approach also creates full, independent copies of the data, just with a different syntax.

---

1. WHAT IS `clone()` DOING?
- A `String` is composed of a pointer, length, and capacity on the stack, which points to the actual character data on the HEAP.
- Calling `.clone()` on a `String` performs a DEEP COPY.
- It allocates new memory on the heap and copies the character data from the original string into this new allocation.
- The result is a brand-new `String` that is completely independent of the original.

---

2. WHY ARE THE ORIGINALS STILL USABLE?
- Because you created full duplicates, the original variables (`pepperoni`, `mushroom`, etc.) still own their original heap data.
- The vector `pizza_toppings` owns the new, cloned strings pushed into it.
- You now have two sets of owned `String` data.

---

3. PERFORMANCE & MEMORY COST
- This is the most memory-intensive option because all string data is duplicated on the heap.
- The cloning process involves heap allocation, which can be slower than simply copying a reference (like with `&str`).

---

4. WHEN TO USE THIS SYNTAX
- The `vec![]` macro (from Option 1) is great for initializing a vector all at once.
- Using `.push()` is more flexible and is what you would use when:
  - Adding items to a vector inside a loop.
  - Adding items conditionally (e.g., inside an `if` statement).
  - Building a vector dynamically piece by piece.
*/