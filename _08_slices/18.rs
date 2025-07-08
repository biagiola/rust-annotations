fn main() {
    let mut values: [i32; 6] = [4, 8, 15, 16, 23, 42];

    let regular_reference: &[i32; 6] = &values;  // Immutable borrow of entire array
    let slice_of_three: &[i32] = &values[..3];   // Immutable borrow of part of array

    values[0] = 99; // ❌ FAILS: Cannot modify while borrowed
    println!("{:?}", values);

    // println!("{:?}", regular_reference); 
    // println!("{:?}", slice_of_three);
}

// How to Fix It:
// Option 1: Use references after modification:
// fn main() {
//     let mut values: [i32; 6] = [4, 8, 15, 16, 23, 42];
//    
//     values[0] = 99; // ✅ Modify first
//    
//     let regular_reference: &[i32; 6] = &values;
//     let slice_of_three: &[i32] = &values[..3];
//    
//     println!("{:?}", values);
//     println!("{:?}", slice_of_three);
// }

// Option 2: Scope the borrows:
// fn main() {
//     let mut values: [i32; 6] = [4, 8, 15, 16, 23, 42];
//    
//     {
//         let slice_of_three: &[i32] = &values[..3];
//         println!("{:?}", slice_of_three);
//     } // slice_of_three goes out of scope here
//    
//     values[0] = 99; // ✅ Now we can modify
//     println!("{:?}", values);
// }

// Option 3: Clone the data:
// fn main() {
//     let mut values: [i32; 6] = [4, 8, 15, 16, 23, 42];
//    
//     let slice_copy: Vec<i32> = values[..3].to_vec(); // Clone the slice
//    
//     values[0] = 99; // ✅ No borrow, so we can modify
//     println!("{:?}", values);
//     println!("{:?}", slice_copy);
// }