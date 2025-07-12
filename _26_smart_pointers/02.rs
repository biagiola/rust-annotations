// The Role of unsafe (This is the key part)
// unsafe does not prevent a panic. Think of the unsafe block as telling the compiler:
// > "Hey, compiler. I know you can't guarantee the safety of the code inside this block.
// I'm telling you that I, the programmer, have checked it and I am taking full responsibility
// for its safety. Please allow me to perform these dangerous operations."

// When the program executes *sushi_raw_pointer_const_1, it's trying to read from a memory
// address that has been deallocated. This triggers Undefined Behavior (UB).
// Undefined Behavior is much scarier than a panic. A panic is Rust's controlled way
// of shutting down when it detects a known error (like an index out of bounds).
// Undefined Behavior means anything can happen. The compiler makes no promises.
// Examples:
// . The program might crash with a "segmentation fault". This is often the best outcome
// for UB because it's obvious.
// . It might read whatever garbage data happens to be in that memory location now, leading
// to bizarre and incorrect results later in your program.
// . It might seem to work perfectly, only to fail on a different computer, on a different
// day, or with a different compiler version.
// This is the most dangerous outcome because the bug is hidden.

fn main() {
    let mut sushi: String = String::from("Yellowtail");

    let sushi_raw_pointer_const_1: *const String = &raw const sushi;
    let sushi_raw_pointer_const_2: *const String = &sushi; // this is the same as the above line

    let sushi_raw_pointer_mut_1: *mut String = &raw mut sushi;
    let sushi_raw_pointer_mut_2: *mut String = &mut sushi; // this is the same as the above line


    drop(sushi);

    unsafe {
        println!("sushi_raw_pointer_const_1 is: {}", *sushi_raw_pointer_const_1);
        println!("sushi_raw_pointer_const_2 is: {}", *sushi_raw_pointer_const_2);
        
        println!("sushi_raw_pointer_mut_1 is: {}", *sushi_raw_pointer_mut_1);
        println!("sushi_raw_pointer_mut_2 is: {}", *sushi_raw_pointer_mut_2);
    }
}

