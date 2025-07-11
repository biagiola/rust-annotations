// How to keep ownership of the data
// Option 1: Clone the Strings

fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    // Clone the strings - original variables remain usable
    let mut pizza_toppings: Vec<String> = vec![
        pepperoni.clone(),
        mushroom.clone(),
        sausage.clone()
    ];

    pizza_toppings[1] = String::from("Olives");
    println!("{pizza_toppings:#?}");

    // Original variables are still accessible!
    println!("Original pepperoni: {}", pepperoni);
    println!("Original mushroom: {}", mushroom);
    println!("Original sausage: {}", sausage);
}

/*
OPTION 1: CLONE THE STRINGS

This approach creates full, independent copies of the data, ensuring the vector owns its own strings.

---

1. WHAT IS `clone()` DOING?
- A `String` is composed of a pointer, length, and capacity on the stack, which points to the actual character data on the HEAP.
- Calling `.clone()` on a `String` performs a DEEP COPY.
- It allocates new memory on the heap and copies the character data from the original string into this new allocation.
- The result is a brand-new `String` that is completely independent of the original.

---

2. WHY ARE THE ORIGINALS STILL USABLE?
- Because you created full duplicates, the original variables (`pepperoni`, `mushroom`, etc.) still own their original heap data.
- The vector `pizza_toppings` now owns the new, cloned strings.
- You have two separate sets of owned `String` data, one held by the original variables and one held by the vector.

---

3. PERFORMANCE & MEMORY COST
- This is the most memory-intensive option because all string data is duplicated on the heap.
- The cloning process involves heap allocation, which can be slower than simply copying a reference (like with `&str`).

---

4. WHEN TO USE THIS APPROACH
- When you truly need two separate, owned copies of the data.
- When the vector needs to own its data so it can outlive the original variables.
- When you intend to modify the strings in the vector independently from the originals.
*/