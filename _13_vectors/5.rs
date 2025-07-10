fn main() {
    let pepperoni: String = String::from("Pepperoni");
    let mushroom: String = String::from("Mushroom");
    let sausage: String = String::from("Sausage");

    let mut pizza_toppings: Vec<String> = vec![pepperoni, mushroom, sausage];

    // IMMUTABLE REFERENCES - These are fine, you can have multiple immutable refs
    // Using get() method returns Option<&T> - safe way to access elements
    let target_topping: Option<&String> = pizza_toppings.get(2);
    match target_topping {
        Some(value) => println!("Selected topping is: {value}"),
        None => println!("No value at that index position")
    }

    // Multiple immutable references are allowed
    let ref1 = &pizza_toppings[0];
    let ref2 = &pizza_toppings[1];
    let ref3 = &pizza_toppings[2];
    println!("Multiple immutable refs: {ref1}, {ref2}, {ref3}");

    // MUTABLE REFERENCES - Rust's borrowing rules apply here    
    // This is fine - one mutable reference at a time
    {
        let mut_ref = &mut pizza_toppings[2];
        *mut_ref = String::from("Olives");
        println!("Modified topping: {mut_ref}");
    } // mut_ref goes out of scope here

    // This is also fine - the previous mutable reference is no longer active
    {
        let another_mut_ref = &mut pizza_toppings[2];
        *another_mut_ref = String::from("Peppers");
        println!("Modified again: {another_mut_ref}");
    } // another_mut_ref goes out of scope here

    // WHAT WON'T WORK - These would cause compile errors
    
    // COMPILE ERROR: Cannot have multiple mutable references simultaneously
    // let mut_ref1 = &mut pizza_toppings[2];
    // let mut_ref2 = &mut pizza_toppings[2];  // ❌ This would fail
    // println!("{mut_ref1}, {mut_ref2}");

    // COMPILE ERROR: Cannot mix immutable and mutable references
    // let immutable_ref = &pizza_toppings[2];
    // let mutable_ref = &mut pizza_toppings[2];  // ❌ This would fail
    // println!("{immutable_ref}, {mutable_ref}");

    println!("Final pizza toppings: {pizza_toppings:#?}");
}

/*
RUST BORROWING RULES SUMMARY:
1. You can have EITHER:
   - Any number of immutable references (&T)
   - OR exactly ONE mutable reference (&mut T)
   - BUT NOT BOTH at the same time

2. References must always be valid (no dangling references)

3. The borrow checker ensures memory safety by enforcing these rules at compile time

4. When a reference goes out of scope, the borrow ends and you can create new references

Why these rules exist:
- Prevents data races in concurrent code
- Ensures memory safety without garbage collection
- Eliminates use-after-free and double-free errors
*/
