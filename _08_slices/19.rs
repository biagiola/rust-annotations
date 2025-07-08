fn main () {
    let values: [i32; 6] = [4, 8, 15, 16, 23, 42];
    let slice_of_three: &[i32] = &values[..3];
    
    print_length(slice_of_three); // ✅ Works with &[i32] parameter
}
 
// ❌ This would NOT work with our slice:
// fn print_length(reference: &[i32; 6]) { 
//     println!("{}", reference.len());
// }
// Error: expected `&[i32; 6]` but got `&[i32]` (different types!)

// ✅ This DOES work - accepts any slice length:
fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}


// KEY CONCEPT: Fixed-Size Array Reference vs Dynamic Slice Reference
// &[T; N]  - Reference to fixed-size array (compile-time known size)
// &[T]     - Reference to slice (runtime dynamic size)

// Examples

// Fixed-size function (RESTRICTIVE):
fn process_exactly_6(arr: &[i32; 6]) {
    println!("Processing exactly 6 elements: {:?}", arr);
}

// Dynamic slice function (FLEXIBLE):
fn process_any_size(slice: &[i32]) {
    println!("Processing {} elements: {:?}", slice.len(), slice);
}

// Usage examples:
// let full_array: [i32; 6] = [1, 2, 3, 4, 5, 6];
// let slice_3: &[i32] = &full_array[..3];
// let slice_4: &[i32] = &full_array[..4];
// 
// process_exactly_6(&full_array);  // ✅ Works - exact match
// process_exactly_6(slice_3);      // ❌ Error - wrong type
// 
// process_any_size(&full_array);   // ✅ Works - auto-coercion
// process_any_size(slice_3);       // ✅ Works - accepts any slice
// process_any_size(slice_4);       // ✅ Works - accepts any slice

// BEST PRACTICE: Use &[T] for function parameters
// - More flexible: accepts arrays, slices, vectors
// - Better API design: callers don't need specific sizes
// - Idiomatic Rust: standard library uses this pattern