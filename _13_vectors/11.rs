// How to keep ownership of the data
// Option 3: Use String Literals (Most Efficient)

fn main() {
    let pepperoni: &str = "Pepperoni";
    let mushroom: &str = "Mushroom";
    let sausage: &str = "Sausage";

    let mut pizza_toppings: Vec<&str> = vec![pepperoni, mushroom, sausage];

    pizza_toppings[1] = "Olives";
    println!("{pizza_toppings:#?}");

    // Original variables are still accessible
    println!("{pepperoni}");
    println!("{mushroom}");
    println!("{sausage}");

}

/*
WHY AREN'T THE ORIGINAL VARIABLES AFFECTED?

This is a key concept about how references and values work in Rust.

1.  STRING LITERALS ARE STORED ELSEWHERE
    The actual string data ("Pepperoni", "Mushroom", etc.) is stored once in a read-only section of your compiled program.
    This data has a 'static lifetime.

2.  VARIABLES HOLD REFERENCES (POINTERS)
    The variables `pepperoni`, `mushroom`, and `sausage` don't hold the string data itself.
    They hold a reference (like a pointer or an address) to the location of that data in memory.

3.  THE VECTOR STORES COPIES OF THE REFERENCES
    When you create the vector `pizza_toppings`, you are not moving the original variables.
    You are making COPIES of the references they hold. Since `&str` is a `Copy` type, this is a very cheap operation.

    Visually, before the change:
    - `mushroom` variable         -> points to "Mushroom" in memory
    - `pizza_toppings[1]` element -> also points to "Mushroom" in memory

4.  REASSIGNMENT CHANGES THE VECTOR'S REFERENCE, NOT THE ORIGINAL
    When you execute `pizza_toppings[1] = "Olives"`, you are doing one thing:
    - Changing the reference stored at index 1 of the vector to point to the new string literal "Olives".

    Visually, after the change:
    - `mushroom` variable         -> STILL points to "Mushroom" in memory (unaffected)
    - `pizza_toppings[1]` element -> NOW points to "Olives" in memory

The original `mushroom` variable and the vector element `pizza_toppings[1]` were two separate copies of a reference.
Changing one does not affect the other.
*/

/*
OPTION 3 - STRING LITERALS AND IMMUTABILITY

WHY MOST EFFICIENT?
- String literals like "Pepperoni" are stored in the binary's READ-ONLY memory
- They have 'static lifetime (live for entire program duration)
- No heap allocation needed - zero memory overhead
- No cloning required - all references point to the same memory location

WHAT DOES "WHEN THE STRINGS DON'T CHANGE" MEAN?
- String literals are IMMUTABLE by nature
- You can't modify the actual string data: "Pepperoni" will always be "Pepperoni"
- You CAN reassign the reference: pizza_toppings[1] = "Olives" works
- But you CAN'T modify the underlying string: you can't change "Pepperoni" to "Pepper"

IMMUTABLE vs MUTABLE REFERENCES:
- pizza_toppings[1] = "Olives" ✅ (reassigning reference)
- pizza_toppings[1].push_str("s") ❌ (trying to modify immutable data)

WHEN TO USE THIS APPROACH:
✅ When your strings are known at compile time
✅ When you don't need to modify the string contents
✅ When you want maximum performance
✅ When you're dealing with static text (like menu items, categories, etc.)

COMPARISON:
- String: Mutable, heap-allocated, owned
- &str: Immutable, stored in binary, borrowed
*/
