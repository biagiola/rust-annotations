// Lecture: Passing Functions to Fn Trait Parameters
// 
// Key Concept: Fn traits don't require closures - they accept ANY callable with the right signature!
// This includes both closures AND regular functions.
//
// Many beginners assume Fn traits are "closure-only," but they're actually trait bounds
// for anything callable with a specific signature. Let's demonstrate this with examples.

// This function accepts anything that implements FnMut() - both functions and closures work!
fn execute_thrice<F>(mut procedure: F)
where
    F: FnMut(), // FnMut allows the callable to be invoked multiple times and potentially mutate captured data
{
    procedure();
    procedure();
    procedure();
}

// A regular function - no closure, no capturing, just plain old function
fn say_hello() {
    println!("Hello from a regular function!");
}

// Another function that we can use in higher-order contexts
fn print_rust() {
    println!("Rust is awesome!");
}

fn main() {
    println!("=== Demonstrating Functions vs Closures with Fn Traits ===\n");

    // Example 1: Passing a regular FUNCTION to a trait bound expecting FnMut
    println!("1. Using a regular function:");
    execute_thrice(say_hello); // Function pointer automatically implements FnMut
    
    println!();

    // Example 2: Passing a CLOSURE to the same trait bound
    println!("2. Using a closure:");
    let mut counter = 0;
    execute_thrice(|| {
        counter += 1;
        println!("Closure call #{}", counter);
    });

    println!();

    // Example 3: Standard library example - Vec::new is a function, not a closure
    println!("3. Real-world example with Option::unwrap_or_else:");
    let option: Option<Vec<String>> = None;
    // Vec::new is a regular function being passed where FnOnce is expected
    let collection = option.unwrap_or_else(Vec::new);
    println!("Created empty collection: {:?}", collection);

    println!();

    // Example 4: Comparing function vs closure for the same task
    println!("4. Function vs closure doing the same thing:");
    
    // Using a closure
    let numbers1 = vec![1, 2, 3, 4, 5];
    let doubled1: Vec<i32> = numbers1.iter().map(|&x| x * 2).collect();
    println!("With closure: {:?}", doubled1);
    
    // Using a named function
    fn double(x: &i32) -> i32 { x * 2 }
    let numbers2 = vec![1, 2, 3, 4, 5];
    let doubled2: Vec<i32> = numbers2.iter().map(double).collect();
    println!("With function: {:?}", doubled2);

    println!("\n=== Key Takeaway ===");
    println!("Fn traits are about 'callable with signature X', not 'closures only'!");
}