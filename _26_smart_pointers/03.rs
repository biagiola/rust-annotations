// How to handle dangling pointers?
// This is the absolute core of the problem, and you've asked the most important question
// about unsafe code. The direct and honest answer is: You can't.
// Once the sushi variable is dropped, there is no reliable way for you to programmatically
// check if the raw pointer is still pointing to a valid String or to garbage.

// Why You Can't Check. Think of it like this:
// . You have the address to a house written on a piece of paper (the raw pointer).
// . The city demolishes the house (the drop(sushi) call).
// . You still have the piece of paper with the address.
// Now, if you drive to that address, what do you see?
// An empty lot?
// A pile of rubble?
// The beginnings of a new building?
// A completely different building that was put up overnight?

// The address on your paper hasn't changed, but the data at that address is gone, and the
// memory has been returned to the operating system. The OS is free to reuse that memory for
// anything else immediately.
// Any check you perform on the "garbage" data at that memory location is meaningless, because
// it could be anything. It could even, by sheer coincidence, have bytes that look like a valid
// String for a microsecond, leading to an even more subtle and dangerous bug.

// This is the very definition of Undefined Behavior. So, How Is This Ever Handled?
// Since you cannot detect a dangling pointer after the fact, the only way to write correct unsafe
// code is to design your program to guarantee it never happens.
// You don't solve it with a runtime check. You solve it with a contract and a better design.
// In Rust, this is almost always done by using Smart Pointers, which are designed specifically
// to handle these ownership "contracts" for you. That's is a topic for the next section.

fn main() {
    let mut sushi: String = String::from("Yellowtail");

    let sushi_raw_pointer_const: *const String = &raw const sushi;
    let sushi_raw_pointer_mut: *mut String = &raw mut sushi;

    drop(sushi);

    unsafe {
        println!("sushi_raw_pointer_const is: {}", *sushi_raw_pointer_const);
        println!("sushi_raw_pointer_mut is: {}", *sushi_raw_pointer_mut);
    }
}